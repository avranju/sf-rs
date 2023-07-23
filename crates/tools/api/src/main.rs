use std::{path::PathBuf, fs, process::Command};

use anyhow::Result;
use windows_metadata::reader::File;

fn main() -> Result<()> {
    let files = File::with_default(&[".windows/winmd/sf.winmd"])?;
    let output_path = PathBuf::from("src/bindings.rs");
    if output_path.exists() {
        fs::remove_file(&output_path)?;
    }

    fs::write(&output_path, windows_bindgen::component("Microsoft.Fabric", &files))?;

    let mut child = Command::new("rustfmt").args([&output_path]).spawn()?;
    child.wait()?;

    Ok(())
}
