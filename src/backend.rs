use crate::constants::{IMAGE, IMAGE_DIGEST};

use std::path::Path;

use thiserror;

pub trait ContainerBackend {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn run(&mut self, dir: &Path) -> Result<(), Self::Error>;
}
