use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../proto")
        .canonicalize()?;
    let file = root.join("altair/v1/altair.proto");
    println!("cargo:rerun-if-changed={}", file.display());

    let fds = protox::compile([&file], [&root])?;
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_fds(fds)?;
    Ok(())
}
