use std::fmt::{self, Display, Formatter};

use docker_api::{models::ImageBuildChunk, opts::PullOpts};
use futures_util::{Stream, StreamExt};

use super::ImageHelper;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PullStatus {
    Downloading,
    Extracting,
}

impl Display for PullStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            PullStatus::Downloading => f.write_str("Downloading"),
            PullStatus::Extracting => f.write_str("Extracting"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PullEvent {
    Error(String),

    Status {
        id: String,
        status: PullStatus,
        current: u64,
        total: u64,
        progress: u64,
    },
}

impl ImageHelper {
    pub fn pull<T>(&self, image: T) -> impl Stream<Item = PullEvent>
    where
        T: AsRef<str>,
    {
        let image_split = image.as_ref().split(":").collect::<Vec<&str>>();
        let image = image_split.get(0).unwrap();
        let mut builder = PullOpts::builder().image(image);

        if let Some(tag) = image_split.get(1) {
            builder = builder.tag(tag);
        }

        let opts = builder.build();
        let imgs = self.docker.images();

        stream! {
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

                            let status = match status.as_str() {
                                "Downloading" => PullStatus::Downloading,
                                "Extracting" => PullStatus::Extracting,
                                _ => return yield PullEvent::Error(String::from("Status must be either Downloading or Extracting!")),
                            };

                            yield PullEvent::Status {
                                id: id.unwrap(),
                                status,
                                total,
                                current: cur,
                                progress: percent,
                            };
                        }

                        ImageBuildChunk::Error {
                            error: _,
                            error_detail,
                        } => {
                            yield PullEvent::Error(error_detail.message);
                        }

                        _ => {}
                    },

                    Err(e) => {
                        yield PullEvent::Error(e.to_string());
                    },
                };
            }
        }
    }
}
