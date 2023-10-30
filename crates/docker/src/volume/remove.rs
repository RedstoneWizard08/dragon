use anyhow::Result;

use super::VolumeHelper;

impl VolumeHelper {
    pub async fn remove<T>(&self, id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let volumes = self.docker.volumes();
        let volume = volumes.get(id.as_ref());

        volume.delete().await.map_err(|v| anyhow!(v))
    }
}
