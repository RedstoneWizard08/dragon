use std::{
    env::temp_dir,
    os::fd::{AsRawFd, FromRawFd},
    process::{ExitStatus, Stdio},
};

use include_dir::Dir;
use tokio::{fs::File, process::Command};
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

pub const CLIENT_DIR: Option<Dir<'static>> = None;

pub async unsafe fn start_client() -> ExitStatus {
    let dir = format!("{}/../client", env!("CARGO_MANIFEST_DIR"));

    let stdout_file = File::create(temp_dir().join("dragon_frontend_stdout.log"))
        .await
        .unwrap();

    let stderr_file = File::create(temp_dir().join("dragon_frontend_stderr.log"))
        .await
        .unwrap();

    let mut cmd = Command::new("pnpm")
        .arg("dev")
        .current_dir(dir)
        .stdout(Stdio::from_raw_fd(stdout_file.as_raw_fd()))
        .stderr(Stdio::from_raw_fd(stderr_file.as_raw_fd()))
        .spawn()
        .unwrap();

    let mut stdout = ReaderStream::new(stdout_file);
    let mut stderr = ReaderStream::new(stderr_file);

    tokio::spawn(async move {
        let mut started = true;

        while let Some(Ok(line)) = stdout.next().await {
            let line = String::from_utf8(line.to_vec()).unwrap();

            if started {
                info!("Client server: {}", line);
            }

            if line.trim().starts_with("- Local:") {
                info!("Client server started!");
                started = true;
            }
        }
    });

    tokio::spawn(async move {
        while let Some(Ok(line)) = stderr.next().await {
            let line = String::from_utf8(line.to_vec()).unwrap();

            error!("Client server: {}", line);
        }
    });

    cmd.wait().await.unwrap()
}

pub fn start() {
    info!("Starting client...");

    tokio::spawn(async { unsafe { start_client().await } });

    info!("Started client!");
}
