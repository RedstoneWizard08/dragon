use anyhow::Result;
use docker_api::opts::ContainerRestartOpts;

use super::ContainerHelper;

impl ContainerHelper {
    pub async fn restart<T>(&self, id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let opts = ContainerRestartOpts::default();
        let containers = self.docker.containers();
        let container = containers.get(id.as_ref());

        container
            .restart(&opts)
            .await
            .map(|_| ())
            .map_err(|v| anyhow!(v))
    }
}
