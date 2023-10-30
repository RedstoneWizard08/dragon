use anyhow::Result;
use docker_api::opts::ImageRemoveOpts;

use super::ImageHelper;

impl ImageHelper {
    pub async fn remove<T>(&self, image: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let opts = ImageRemoveOpts::default();
        let imgs = self.docker.images();
        let img = imgs.get(image.as_ref());

        img.remove(&opts).await.map(|_| ()).map_err(|v| anyhow!(v))
    }
}
