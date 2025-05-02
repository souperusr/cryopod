use std::path::PathBuf;

use tokio::net::unix::uid_t;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use serde_json as json;

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
    PodmanSockConnect(#[source] std::io::Error),

    #[error("Unable to send request to podman")]
    SendRequest(#[from] Box<dyn std::error::Error + Send + Sync>),
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

    pub async fn send_request(&mut self, request_path: &str) -> Result<json::Value, Error> {
        let request = format!(
            "GET /v5.0.0{} HTTP/1.1\r\n\
            Host: localhost\r\n\
            \r\n",
            request_path
        );

        self.0.write_all(&request.into_bytes())
            .await
            .map_err(|err| Error::SendRequest(Box::new(err)))?;

        let mut response: String = String::new();
        self.0.read_to_string(&mut response)
            .await
            .map_err(|err| Error::SendRequest(Box::new(err)))?;

        let res_body = response.splitn(2, "\r\n\r\n")
            .nth(1)
            .unwrap_or("");

        let json: json::Value = serde_json::from_str(&res_body)
            .map_err(|err| Error::SendRequest(Box::new(err)))?;

        return Ok(json);
    }
}
