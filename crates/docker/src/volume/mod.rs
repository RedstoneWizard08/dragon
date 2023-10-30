//! The volume module.
//! This houses a struct (VolumeHelper) that does everything with volumes.
//! Available actions:
//! - backup - Create a backup of the volume and return its location.
//! - create - Create a volume and return its ID.
//! - remove - Remove a volume.

use anyhow::Result;
use docker_api::Docker;

use crate::connect;

pub mod backup;
pub mod create;
pub mod remove;

#[derive(Debug, Clone)]
pub struct VolumeHelper {
    pub(crate) docker: Docker,
}

impl VolumeHelper {
    pub fn with_docker(docker: Docker) -> Self {
        Self { docker }
    }

    pub fn new() -> Result<Self> {
        Ok(Self { docker: connect()? })
    }
}
