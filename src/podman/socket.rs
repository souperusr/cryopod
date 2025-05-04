use super::request::*;

use std::path::PathBuf;

use tokio::net::unix::uid_t;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use serde_json as json;

const API_VERSION: &'static str = "v5.0.0";

/// An opaque handle to the podman.sock file. Use .get() to fetch a handle, and Into<PathBuf> to access the path.
enum PodmanSockFileHandle {
    Host,
    User(uid_t)
}

impl PodmanSockFileHandle {
    /// Tries to fetch a socket handle, first from the host, and then from any user. Returns the socket handle, or None if it cannot be found.
    pub fn get() -> Option<Self> {
        let host_sock_path: PathBuf = (Self::Host).into();

        if host_sock_path.exists() {
            return Some(Self::Host)
        } else {
            // This function has not thread-safe POSIX calls, so we must invoke unsafe. However, we can consider this safe as long as no other invocations of all_users() (or likewise) is called, and no .await is called during the lifetime of the following iterator.
            let mut users = unsafe { users::all_users() };

            // Find any user who owns a valid podman socket.
            let user_sock_uid = users.find(|user| {
                let podman_user_sock: PathBuf = PodmanSockFileHandle::User(user.uid()).into();
                return podman_user_sock.exists()
            })?.uid();

            return Some(PodmanSockFileHandle::User(user_sock_uid));
        }
    }
}

impl Into<PathBuf> for PodmanSockFileHandle {
    fn into(self) -> PathBuf {
        match self {
            Self::Host => return PathBuf::from("/var/run/podman/podman.sock"),
            Self::User(uid) => return PathBuf::from(format!("/var/run/user/{}/podman/podman.sock", uid)),
        }
    }
}

/// An error type for PodmanSocket.
#[derive(Debug, thiserror::Error)]
pub enum PodmanSockErr {
    #[error("No podman socket found. Make sure you have podman installed, and 'podman.sock' is available.")]
    NoPodmanSock,

    #[error("Cannot connect to podman socket")]
    PodmanSockConnect(#[from] std::io::Error),

    #[error("Unable to parse podman response")]
    ParseHttp(#[from] httparse::Error),

    #[error("No body found in podman response")]
    NoHttpBody,

    #[error("Unable to parse podman response")]
    ParseJson(#[from] json::Error)
}

type Error = PodmanSockErr;

/// The socket to connect to podman. Abstracts over UnixStream, and provides helper methods for sending REST requests.
pub struct PodmanSocket(UnixStream);

impl PodmanSocket {
    pub async fn new() -> Result<Self, PodmanSockErr> {
        // Fetch a socket file handle.
        let sock_handle = PodmanSockFileHandle::get().ok_or(Error::NoPodmanSock)?;

        // Attempt to load the socket into a UnixStream
        let stream = UnixStream::connect::<PathBuf>(sock_handle.into()).await.map_err(|err| Error::PodmanSockConnect(err))?;

        Ok(Self(stream))
    }

    pub async fn send_request<T: PodmanRequest>(&mut self, request: T) -> Result<PodmanResponse<T::Response>, Error> {
        let full_request = format!(
            "{} /{}/libpod{} HTTP/1.1\r\n\
            Host: localhost\r\n\
            \r\n",
            T::REQUEST_METHOD,
            API_VERSION,
            request.get_request()
        ).into_bytes();

        self.0.write_all(&full_request).await?;

        let mut response_bytes: Vec<u8> = Vec::with_capacity(512);
        loop {
            let n_bytes = self.0.read_buf(&mut response_bytes).await?;
            if n_bytes == 0 {
                break;
            }
        }

        // Parse http response
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut response = httparse::Response::new(&mut headers);
        let _ = response.parse(&response_bytes)?;

        let headers_end = response_bytes
            .windows(4)
            .position(|seq| seq == b"\r\n\r\n")
            .ok_or(Error::NoHttpBody)? + 4;

        let body = &response_bytes[headers_end..];
        let status_code = response.code.ok_or(Error::NoHttpBody)?; //TODO change error type here

        if status_code >= 200 && status_code < 300 {
            let response: T::Response = json::from_slice(body)?;
            Ok(PodmanResponse { status_code, response: Ok(response)})
        } else {
            let response: ResponseError = json::from_slice(body)?;
            Ok(PodmanResponse { status_code, response: Err(response) })
        }
    }
}
