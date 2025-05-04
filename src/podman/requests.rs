// Implementations of PodmanRequest defined in requests.rs.

use super::request::*;
use serde::Deserialize;


/// Request quering whether the given image is loaded within the podman store.
pub struct ImageExists(pub &'static str);

#[derive(Deserialize)]
pub struct ImageExistsResponse();

impl PodmanRequest for ImageExists {
    type Response = ImageExistsResponse;
    const REQUEST_METHOD: RestMethod = RestMethod::Get;

    fn get_request(&self) -> String {
        format!("/images/{}/exists", self.0)
    }
}
