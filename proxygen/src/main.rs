mod exports;
mod proxy;

use crate::exports::DLLFile;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{collections::BTreeSet, path::PathBuf};

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
        dll: PathBuf,
    },
    /// Generate a new proxy DLL project for the given DLL file
    Generate {
        /// Path to the DLL to proxy
        dll: PathBuf,
        /// Path to the DLL proxy project to create.
        project_dir: PathBuf,
    },
    /// Merges the given DLL's new exports into an existing DLL proxy project
    Merge {
        /// Path to the DLL to proxy
        dll: PathBuf,
        /// Path to the proxy project into which new DLL exports should be merged.
        project_dir: PathBuf,
    },
    /// Updates an exisitng DLL proxy project's exports based on the intercepted exports
    Update {
        /// Path to the proxy project to update. The original DLL is expected to have an underscore suffix
        project_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::DumpExports { dll } => {
            let dll_file = DLLFile::new(dll)?;
            for export in dll_file.get_exports()? {
                println!("{}", export.original);
            }
        }
        Commands::Generate { dll, project_dir } => {
            let dll_file = DLLFile::new(dll)?;
            if let Some(dll_name) = dll
                .file_name()
                .expect("Expected path to end with a file name")
                .to_str()
            {
                proxy::create_proxy_project(
                    &dll_file.get_exports()?,
                    dll_name,
                    &project_dir,
                    dll_file.get_arch()?,
                )?;
            } else {
                return Err(anyhow::anyhow!("Failed to get dll name from path"));
            }
        }
        Commands::Merge { dll, project_dir } => {
            let dll_file = DLLFile::new(dll)?;
            proxy::update_proxy_project(&dll_file.get_exports()?, &project_dir.canonicalize()?)?;
        }
        Commands::Update { project_dir } => {
            proxy::update_proxy_project(&BTreeSet::new(), &project_dir.canonicalize()?)?;
        }
    }

    Ok(())
}
