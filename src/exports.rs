use anyhow::Result;
use exe::{ExportDirectory, VecPE, PE};
use std::path::PathBuf;

pub fn get_exports(pe_file: &PathBuf) -> Result<Vec<String>> {
    println!("Getting exports for {}", pe_file.to_string_lossy());
    let pe = VecPE::from_disk_file(pe_file)?;
    println!("Detected arch: {:?}", pe.get_arch()?);
    let export_directory = ExportDirectory::parse(&pe)?;
    let mut results: Vec<String> = export_directory
        .get_export_map(&pe)?
        .iter()
        .map(|(func, _)| String::from(*func).trim().to_string())
        .filter(|f| {
            f != "DllMain"
                && f != "ORIGINAL_FUNCS"
                && f != "ORIG_FUNCS_PTR"
                && f != "wait_dll_proxy_init"
        })
        .collect();
    results.sort();
    Ok(results)
}
