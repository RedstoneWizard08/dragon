use anyhow::Result;
use docker_api::opts::NetworkCreateOpts;

use super::{NetworkDriver, NetworkHelper};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct NetworkCreateOptions {
    /// The network name.
    pub name: String,

    /// The network driver.
    pub driver: NetworkDriver,
}

impl NetworkHelper {
    pub async fn create(&self, opts: NetworkCreateOptions) -> Result<String> {
        let nopts = NetworkCreateOpts::builder(opts.name)
            .driver(opts.driver.as_str())
            .build();

        let networks = self.docker.networks();
        let network = networks.create(&nopts).await?;

        Ok(network.id().to_string())
    }
}
