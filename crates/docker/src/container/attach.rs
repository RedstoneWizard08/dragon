use std::io::Write;

use anyhow::Result;
use docker_api::conn::TtyChunk;
use futures_util::StreamExt;

use super::ContainerHelper;

impl ContainerHelper {
    pub async fn attach<T, O, E>(&self, id: T, stdout: &mut O, stderr: &mut E) -> Result<()>
    where
        T: AsRef<str>,
        O: Write,
        E: Write,
    {
        let containers = self.docker.containers();
        let container = containers.get(id.as_ref());

        let (mut r, mut _w) = container.attach().await?.split();

        while let Some(res) = r.next().await {
            match res {
                Ok(chunk) => match chunk {
                    TtyChunk::StdOut(data) => stdout.write_all(&data.into_boxed_slice())?,
                    TtyChunk::StdErr(data) => stderr.write_all(&data.into_boxed_slice())?,

                    // This should never be stdin, we are reading, not writing!
                    TtyChunk::StdIn(_) => unreachable!(),
                },

                Err(err) => stderr.write_all(format!("{}\n", err.to_string()).as_bytes())?,
            };
        }

        Ok(())
    }
}
