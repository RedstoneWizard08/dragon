use crate::connect;
use anyhow::Result;
use async_trait::async_trait;
use docker_api::{models::ImageBuildChunk, opts::PullOpts, Docker};
use futures_util::StreamExt;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PullEvent {
    Started,
    Completed,

    // An error. :(
    Error(String),

    /// The status.
    /// (id, status, current, total, progress [of 100%])
    /// status will be either "Downloading" or "Extracting"
    Status(String, String, u64, u64, u64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PullWrapperParams {
    pub image: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PullWrapper {
    params: PullWrapperParams,
    docker: Docker,
}

#[async_trait]
impl Wrapper<PullEvent, PullWrapperParams> for PullWrapper {
    async fn exec(&self, tx: Sender<E>) -> Result<()> {
        tx.send(PullEvent::Started)?;

        if image.starts_with("~") {
            tx.send(PullEvent::Completed)?;

            return Ok(());
        }

        let opts = PullOpts::builder().image(self.params.image).build();
        let imgs = self.docker.images();
        let mut stream = imgs.pull(&opts);

        while let Some(item) = stream.next().await {
            match item {
                Ok(val) => match val {
                    ImageBuildChunk::PullStatus {
                        status,
                        id,
                        progress: _,
                        progress_detail,
                    } => {
                        if progress_detail.is_none() {
                            continue;
                        }

                        let cur = progress_detail.clone().unwrap().current.unwrap_or(0);
                        let total = progress_detail.unwrap().total.unwrap_or(0);

                        if cur == 0 || total == 0 {
                            continue;
                        }

                        let percent = (100 * cur) / total;

                        tx.send(PullEvent::Status(id.unwrap(), status, cur, total, percent))?;
                    }

                    ImageBuildChunk::Error {
                        error: _,
                        error_detail,
                    } => {
                        tx.send(PullEvent::Error(error_detail.message))?;
                    }

                    _ => {}
                },

                Err(e) => eprintln!("{}", e),
            };
        }

        tx.send(PullEvent::Completed)?;

        drop(tx);

        Ok(())
    }
}
