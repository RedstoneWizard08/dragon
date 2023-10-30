//! The container module.
//! This houses a struct (ContainerHelper) that does everything with containers.
//! Available actions:
//! - attach - Attaches provided stdout and stderr streams to the container's.
//! - create - Creates a new container and returns its ID.
//! - kill - Kills a container.
//! - remove - Removes a container.
//! - restart - Restarts a container.
//! - run - Creates and starts a container, returning its ID.
//! - start - Starts a container.
//! - stop - Stops a container.

use anyhow::Result;
use docker_api::Docker;

use crate::connect;

pub mod attach;
pub mod create;
pub mod kill;
pub mod remove;
pub mod restart;
pub mod run;
pub mod start;
pub mod stop;

#[derive(Debug, Clone)]
pub struct ContainerHelper {
    pub(crate) docker: Docker,
}

impl ContainerHelper {
    pub fn with_docker(docker: Docker) -> Self {
        Self { docker }
    }

    pub fn new() -> Result<Self> {
        Ok(Self { docker: connect()? })
    }
}
