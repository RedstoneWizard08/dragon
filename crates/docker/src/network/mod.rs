//! The network module.
//! This houses a struct (NetworkHelper) that does everything with networks.
//! Available actions:
//! - create - Creates a network and returns its ID.
//! - modify - Edit a network's properties.
//! - remove - Remove a network.

use std::fmt::{self, Display, Formatter};

use anyhow::Result;
use docker_api::Docker;

use crate::connect;

pub mod create;
pub mod modify;
pub mod remove;

#[derive(Debug, Clone)]
pub struct NetworkHelper {
    pub(crate) docker: Docker,
}

impl NetworkHelper {
    pub fn with_docker(docker: Docker) -> Self {
        Self { docker }
    }

    pub fn new() -> Result<Self> {
        Ok(Self { docker: connect()? })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum NetworkDriver {
    #[default]
    Bridge,
    Host,
    Overlay,
    IPvLAN,
    MACvLAN,
    None,
}

impl NetworkDriver {
    pub fn as_str(self) -> &'static str {
        match self {
            NetworkDriver::Bridge => "bridge",
            NetworkDriver::Host => "host",
            NetworkDriver::IPvLAN => "ipvlan",
            NetworkDriver::MACvLAN => "macvlan",
            NetworkDriver::None => "none",
            NetworkDriver::Overlay => "overlay",
        }
    }
}

impl Display for NetworkDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
