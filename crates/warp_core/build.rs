use anyhow::Result;

fn main() -> Result<()> {
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY")?;

    if target_family != "wasm" {
        println!("cargo:rustc-cfg=feature=\"local_fs\"");
    }

    // Read version from repo root VERSION file and expose as GIT_RELEASE_TAG
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let version_path = std::path::Path::new(&manifest_dir).join("../../VERSION");
    println!("cargo:rerun-if-changed=../../VERSION");
    if let Ok(version) = std::fs::read_to_string(&version_path) {
        println!("cargo:rustc-env=GIT_RELEASE_TAG=v{}", version.trim());
    }

    Ok(())
}
