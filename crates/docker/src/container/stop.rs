use anyhow::Result;
use docker_api::opts::ContainerStopOpts;

use super::ContainerHelper;

impl ContainerHelper {
    pub async fn stop<T>(&self, id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let opts = ContainerStopOpts::default();
        let containers = self.docker.containers();
        let container = containers.get(id.as_ref());

        container
            .stop(&opts)
            .await
            .map(|_| ())
            .map_err(|v| anyhow!(v))
    }
}
