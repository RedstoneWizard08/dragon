use std::{
    io::{BufRead, BufReader},
    process::ExitStatus,
};

use include_dir::Dir;

pub const CLIENT_DIR: Option<Dir<'static>> = None;

pub async unsafe fn start_client() -> ExitStatus {
    let dir = format!("{}/../client", env!("CARGO_MANIFEST_DIR"));

    let cmd = cmd!("pnpm", "dev").dir(dir).stderr_to_stdout();
    let handle = cmd.start().unwrap();

    let reader = cmd.reader().unwrap();
    let mut reader = BufReader::new(reader).lines();

    while let Some(Ok(line)) = reader.next() {
        info!("Client server: {}", line);
    }

    handle.wait().unwrap().status
}

pub fn start() {
    info!("Starting client...");

    tokio::spawn(async { unsafe { start_client().await } });

    info!("Started client!");
}
