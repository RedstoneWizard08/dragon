use anyhow::Result;

use super::{create::ContainerCreateOptions, ContainerHelper};

impl ContainerHelper {
    pub async fn run(&self, opts: ContainerCreateOptions) -> Result<String> {
        let id = self.create(opts).await?;
        let containers = self.docker.containers();
        let container = containers.get(&id);

        container.start().await.map(|_| id).map_err(|v| anyhow!(v))
    }
}
