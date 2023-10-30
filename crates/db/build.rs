fn main() {
    println!(
        "cargo:rerun-if-changed={}/../../migrations",
        env!("CARGO_MANIFEST_DIR")
    );
}
