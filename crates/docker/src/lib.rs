pub mod wrappers;

pub use wrappers::{attach, create, delete, pull, restart, run, start, stop};

use anyhow::Result;
use docker_api::Docker;

#[cfg(unix)]
pub fn connect() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(windows)]
pub fn connect() -> Result<Docker> {
    Ok(Docker::unix("//./pipe/docker_engine"))
}

#[cfg(not(any(unix, windows)))]
pub fn connect() -> Result<Docker> {
    Docker::new("tcp://127.0.0.1:8080")
}

#[cfg(test)]
mod tests;
