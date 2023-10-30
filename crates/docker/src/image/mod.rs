//! The image module.
//! This houses a struct (ImageHelper) that does everything with images.
//! Available actions:
//! - pull - Pulls an image and returns a stream to monitor the status.
//! - remove - Removes an image.

use anyhow::Result;
use docker_api::Docker;

use crate::connect;

pub mod pull;
pub mod remove;

#[derive(Debug, Clone)]
pub struct ImageHelper {
    pub(crate) docker: Docker,
}

impl ImageHelper {
    pub fn with_docker(docker: Docker) -> Self {
        Self { docker }
    }

    pub fn new() -> Result<Self> {
        Ok(Self { docker: connect()? })
    }
}
