use anyhow::Result;

use super::NetworkHelper;

impl NetworkHelper {
    pub async fn remove<T>(&self, id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let networks = self.docker.networks();
        let network = networks.get(id.as_ref());

        network.delete().await.map_err(|v| anyhow!(v))
    }
}
