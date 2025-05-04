/* --- A podman implementation of ContainerBackend -------- */

mod socket;
mod request;
mod requests;

use crate::constants;
use crate::podman::socket::{PodmanSocket, PodmanSockErr};

use serde_json as json;

use users::uid_t;

/* --- PodmanBackend -------------------------- */

pub struct PodmanBackend {
    socket: PodmanSocket,
}

impl PodmanBackend {
    pub async fn new() -> Result<Self, PodmanSockErr> {
        let socket = PodmanSocket::new().await?;

        Ok(Self {
            socket
        })
    }

    pub async fn run(&mut self) -> Result<(), PodmanSockErr> {
        self.is_image_loaded(constants::IMAGE_DIGEST).await?;
        return Ok(())
    }

    /* - Private implementation - */

    async fn is_image_loaded(&mut self, image_digest: &'static str) -> Result<bool, PodmanSockErr> {
        let req = requests::ImageExists(image_digest);

        match self.socket.send_request(req).await?.response {
            Ok(resp) => println!("IT WORKED"),
            Err(err_resp) => println!("IT DIDN'T WORK IN A GOOD WAY! {}", err_resp.message),
        };


        todo!()
    }
}
