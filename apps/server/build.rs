extern crate cfg_if;

#[cfg(not(debug_assertions))]
use std::process::Command;

#[cfg(not(debug_assertions))]
fn run_build() {
    let dir = format!("{}/../client", env!("CARGO_MANIFEST_DIR"));

    println!("cargo:rerun-if-changed={}", dir);

    assert!(Command::new("pnpm")
        .arg("build")
        .current_dir(dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(not(debug_assertions))] {
            run_build();
        }
    };
}
