/* --- A podman implementation of ContainerBackend -------- */

mod socket;

use crate::ContainerBackend;
use crate::constants;
use crate::podman::socket::{PodmanSocket, PodmanSockErr};

use std::path::{Path, PathBuf};
use serde_json as json;

use users::uid_t;

/* --- PodmanBackendError ------------------------ */

#[derive(Debug, thiserror::Error)]
pub enum PodmanBackendError {
    #[error("Unable to connect to podman")]
    PodmanConnect(#[source] PodmanSockErr),

    #[error("Unable to load image")]
    ImageLoaded(#[source] PodmanSockErr),

    #[error("Bad request")]
    BadRequest,
}

type Error = PodmanBackendError;

/* --- PodmanBackend -------------------------- */

pub struct PodmanBackend {
    socket: PodmanSocket,
}

impl PodmanBackend {
    pub async fn new() -> Result<Self, Error> {
        let socket = PodmanSocket::new().await.map_err(|err| Error::PodmanConnect(err))?;

        Ok(Self {
            socket
        })
    }

    /* - Private implementation - */

    async fn is_image_loaded(&mut self, image_digest: &str) -> Result<bool, Error> {
        let msg = format!("/libpod/images/{}/get", constants::IMAGE_DIGEST);
        let res = self.socket.send_request(&msg).await.map_err(|err| Error::ImageLoaded(err))?;

        if let json::Value::Number(n) = &res["response"] {
            let return_code = n.as_u64();

            match return_code {
                Some(204) => return Ok(true),
                Some(404) => return Ok(false),
                Some(500) => return Err(Error::BadRequest),
                _   => unreachable!()
            }
        } else {
            Err(Error::BadRequest)
        }

        // match &res["response"] {
        //     json::Value::Number(val) if val == 204.into() => return Ok(true),
        //     json::Value::Number(val) if val == 404.into() => return Ok(false),
        //     json::Value::Number(val) if val == 500.into() => return Err(Error::BadRequest),
        //     _   => unreachable!()
        // }
    }

    async fn load_image(&self) -> Result<(), Error> {
        todo!()
    }
}

impl ContainerBackend for PodmanBackend {
    type Error = PodmanBackendError;

    async fn run(&mut self, dir: &Path) -> Result<(), Error> {
        // Ensure image is loaded
        if !self.is_image_loaded(constants::IMAGE_DIGEST).await? {
            self.load_image().await?;
        }

        todo!();
    }
}
