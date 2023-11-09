use anyhow::{Ok, Result};
use exe::{ExportDirectory, VecPE, PE, Arch};
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

pub struct DLLFile {
    path: PathBuf,
    pe_file: VecPE,
}

impl DLLFile {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let pe_file = VecPE::from_disk_file(path)?;
        Ok(Self {
            path: path.clone(),
            pe_file: pe_file,
        })
    }

    pub fn get_exports(&self) -> Result<BTreeSet<ExportName>> {
        println!("Getting exports for {}", self.path.to_string_lossy());
        println!("Detected arch: {:?}", self.pe_file.get_arch()?);
        let export_directory = ExportDirectory::parse(&self.pe_file)?;
        Ok(export_directory
            .get_export_map(&self.pe_file)?
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

    pub fn get_arch(&self) -> Result<Arch> {
        Ok(self.pe_file.get_arch()?)
    }
}