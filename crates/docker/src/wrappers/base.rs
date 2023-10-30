use std::sync::mpsc::Sender;

use anyhow::Result;
use async_trait::async_trait;
use crate::connect;

#[async_trait]
pub trait Wrapper<E, P> {
    async fn exec(&self, tx: Sender<E>) -> Result<()>;

    fn new(params: P) -> Result<Self> {
        Ok(Self {
            params,
            docker: connect()?,
        })
    }
}
