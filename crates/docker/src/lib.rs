#![feature(async_closure)]

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate async_stream;

pub mod container;
pub mod image;
pub mod network;
pub mod volume;

pub use container::ContainerHelper;
pub use image::ImageHelper;
pub use network::NetworkHelper;
pub use volume::VolumeHelper;

use anyhow::Result;
use docker_api::Docker;

#[cfg(unix)]
pub(crate) fn connect() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(windows)]
pub(crate) fn connect() -> Result<Docker> {
    Ok(Docker::unix("//./pipe/docker_engine"))
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn connect() -> Result<Docker> {
    Docker::new("tcp://127.0.0.1:8080")
}

#[cfg(test)]
mod tests;
