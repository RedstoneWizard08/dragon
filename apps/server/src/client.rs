use std::process::{ExitStatus, Stdio};

use include_dir::Dir;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

pub const CLIENT_DIR: Option<Dir<'static>> = None;
pub static mut CLIENT_READY: bool = false;

pub fn read_lines(child: &mut Child) {
    let stdout = child
        .stdout
        .take()
        .expect("Child did not have a handle to stdout!");

    let stderr = child
        .stderr
        .take()
        .expect("Child did not have a handle to stderr!");

    let mut stdout = BufReader::new(stdout).lines();
    let mut stderr = BufReader::new(stderr).lines();

    tokio::spawn(async move {
        while let Ok(Some(line)) = stdout.next_line().await {
            unsafe {
                if CLIENT_READY {
                    info!("Client: {}", line);
                }
            }

            if line.trim().starts_with("✓ Ready in") {
                info!("Started client!");

                unsafe {
                    CLIENT_READY = true;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Ok(Some(line)) = stderr.next_line().await {
            error!("Client: {}", line);
        }
    });
}

pub async fn start_client() -> ExitStatus {
    let dir = format!("{}/../client", env!("CARGO_MANIFEST_DIR"));

    let mut cmd = Command::new("pnpm")
        .arg("dev")
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    read_lines(&mut cmd);

    cmd.wait().await.unwrap()
}

pub fn start() {
    info!("Starting client...");

    tokio::spawn(async { start_client().await });
}
