use anyhow::Result;
use docker_api::opts::ContainerRemoveOpts;

use super::ContainerHelper;

impl ContainerHelper {
    pub async fn remove<T>(&self, id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let opts = ContainerRemoveOpts::default();
        let containers = self.docker.containers();
        let container = containers.get(id.as_ref());

        container
            .remove(&opts)
            .await
            .map(|_| ())
            .map_err(|v| anyhow!(v))
    }
}
