use anyhow::Result;

use super::ContainerHelper;

impl ContainerHelper {
    pub async fn kill<T>(&self, id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let containers = self.docker.containers();
        let container = containers.get(id.as_ref());

        container
            .kill(None)
            .await
            .map(|_| ())
            .map_err(|v| anyhow!(v))
    }
}
