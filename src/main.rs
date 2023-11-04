mod exports;
mod proxy;

use crate::exports::get_exports;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A DLL export dumper and proxy generator
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints out the exported functions from a given PE file
    DumpExports {
        /// Path to the DLL to dump exports from
        file: PathBuf,
    },
    GenProxy {
        /// Path to the DLL to proxy
        file: PathBuf,
        /// Path to the proxy project to create. The original DLL is expected to have an underscore suffix
        project_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::DumpExports { file } => {
            let exports = get_exports(file)?;
            for export in exports.iter() {
                println!("{}", export);
            }
        }
        Commands::GenProxy { file, project_dir } => {
            let exports = get_exports(file)?;
            if let Some(dll_name) = file
                .file_name()
                .expect("Expected path to end with a file name")
                .to_str()
            {
                proxy::create_proxy_project(&exports, dll_name, project_dir)?;
            }
            else {
                return Err(anyhow::anyhow!("Failed to get dll name from path"));
            }
        }
    }

    Ok(())
}
