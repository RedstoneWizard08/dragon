use anyhow::Result;

use super::VolumeHelper;

impl VolumeHelper {
    pub async fn backup<T>(&self, _id: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        todo!()
    }
}
