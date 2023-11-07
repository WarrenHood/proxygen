use anyhow::Result;
use exe::{ExportDirectory, VecPE, PE};
use std::{collections::BTreeSet, path::PathBuf};

pub fn get_exports(pe_file: &PathBuf) -> Result<BTreeSet<String>> {
    println!("Getting exports for {}", pe_file.to_string_lossy());
    let pe = VecPE::from_disk_file(pe_file)?;
    println!("Detected arch: {:?}", pe.get_arch()?);
    let export_directory = ExportDirectory::parse(&pe)?;
    Ok(export_directory
        .get_export_map(&pe)?
        .into_iter()
        .map(|(func, _)| String::from(func).trim().to_string())
        .map(|f| match f.split_once('@') {
            Some((func_name, _)) => func_name.into(),
            None => f,
        })
        .filter(|f| {
            f != "DllMain"
                && f != "ORIGINAL_FUNCS"
                && f != "ORIG_FUNCS_PTR"
                && f != "wait_dll_proxy_init"
        })
        .collect())
}
