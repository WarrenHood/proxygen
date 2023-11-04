use anyhow::Result;
use std::{path::PathBuf, process::Command};

pub fn create_proxy_project(
    _exports: &Vec<String>,
    _dll_name: impl Into<String>,
    _out_dir: &PathBuf,
) -> Result<()> {
    unimplemented!();
    // if out_dir.exists() {
    //     return Err(anyhow::anyhow!(
    //         "Folder {} already exists. Aborting",
    //         out_dir.to_string_lossy()
    //     ));
    // }
    // let status = Command::new("cargo")
    //     .arg("new")
    //     .arg("--lib")
    //     .arg(out_dir)
    //     .status()?;
    // if status.success() {
    //     println!(
    //         "Successfully created new proxy project at {}",
    //         out_dir.to_string_lossy()
    //     );
    // } else {
    //     return Err(anyhow::anyhow!(
    //         "Failed to create new proxy project at {}. Exit code: {}",
    //         out_dir.to_string_lossy(),
    //         match status.code() {
    //             Some(code) => format!("{code}"),
    //             None => "None".into(),
    //         }
    //     ));
    // }
    // Ok(())
}
