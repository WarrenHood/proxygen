use anyhow::Result;
use exe::{ExportDirectory, VecPE, PE};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ExportName {
    pub original: String,
    pub cleaned: String,
}

impl ExportName {
    pub fn new(func_name: &str) -> Self {
        let func_name = func_name.trim();
        Self {
            original: func_name.into(),
            cleaned: clean_func_name(func_name),
        }
    }
}

/// Cleans up exported function names
fn clean_func_name(func_name: &str) -> String {
    // TODO: In the future it'd probably be better to properly demangle symbols
    func_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn get_exports(pe_file: &PathBuf) -> Result<BTreeSet<ExportName>> {
    println!("Getting exports for {}", pe_file.to_string_lossy());
    let pe = VecPE::from_disk_file(pe_file)?;
    println!("Detected arch: {:?}", pe.get_arch()?);
    let export_directory = ExportDirectory::parse(&pe)?;
    Ok(export_directory
        .get_export_map(&pe)?
        .into_iter()
        .map(|(func, _)| ExportName::new(func))
        .filter(|f| {
            f.cleaned != "DllMain"
                && f.cleaned != "ORIGINAL_FUNCS"
                && f.cleaned != "ORIG_FUNCS_PTR"
                && f.cleaned != "wait_dll_proxy_init"
        })
        .collect())
}
