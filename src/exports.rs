use anyhow::Result;
use exe::{ExportDirectory, VecPE};
use std::path::PathBuf;

pub fn get_exports(pe_file: &PathBuf) -> Result<Vec<String>> {
    let pe = VecPE::from_disk_file(pe_file)?;
    let export_directory = ExportDirectory::parse(&pe)?;
    let mut results: Vec<String> = export_directory
        .get_export_map(&pe)?
        .iter()
        .map(|(func, _)| String::from(*func))
        .collect();
    results.sort();
    Ok(results)
}