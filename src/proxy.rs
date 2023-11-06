use anyhow::{Ok, Result};
use regex::Regex;
use std::{path::PathBuf, process::Command};
use tera::{Context, Tera};

const CARGO_TEMPLATE: &str = include_str!("templates/Cargo.toml");
const EXPORT_INDICES_TEMPLATE: &str = include_str!("templates/export_indices.rs");
const INTERCEPTED_EXPORTS_TEMPLATE: &str = include_str!("templates/intercepted_exports.rs");
const LIB_TEMPLATE: &str = include_str!("templates/lib.rs");
const ORIG_EXPORTS_TEMPLATE: &str = include_str!("templates/orig_exports.rs");
const PROXIED_EXPORTS_TEMPLATE: &str = include_str!("templates/proxied_exports.rs");

struct ProxyTemplates {
    tera: Tera,
}

impl ProxyTemplates {
    pub fn new() -> Result<Self> {
        let mut tera = Tera::new("templates/**/*")?;
        tera.add_raw_template("Cargo.toml", CARGO_TEMPLATE)?;
        tera.add_raw_template("export_indices.rs", EXPORT_INDICES_TEMPLATE)?;
        tera.add_raw_template("intercepted_exports.rs", INTERCEPTED_EXPORTS_TEMPLATE)?;
        tera.add_raw_template("lib.rs", LIB_TEMPLATE)?;
        tera.add_raw_template("orig_exports.rs", ORIG_EXPORTS_TEMPLATE)?;
        tera.add_raw_template("proxied_exports.rs", PROXIED_EXPORTS_TEMPLATE)?;

        Ok(Self { tera })
    }

    pub fn get_cargo_toml(
        &self,
        package_name: impl Into<String>,
        target_dll_name: impl Into<String>,
    ) -> Result<String> {
        let mut ctx = Context::new();
        ctx.insert("package_name", &package_name.into());
        ctx.insert("target_dll_name", &target_dll_name.into());
        Ok(self.tera.render("Cargo.toml", &ctx)?)
    }

    pub fn get_export_indices(&self, exports: &Vec<String>) -> Result<String> {
        let mut ctx = Context::new();
        let export_indices: String = exports
            .iter()
            .enumerate()
            .map(|(i, export_name)| format!("pub const Index_{}: usize = {};", export_name, i))
            .fold(String::new(), |acc, x| acc + "\n" + &x)
            .trim_start()
            .into();
        ctx.insert("export_indices", &export_indices);
        ctx.insert("total_exports", &exports.len());
        Ok(self.tera.render("export_indices.rs", &ctx)?)
    }

    pub fn get_lib(&self, package_name: impl Into<String>) -> Result<String> {
        let mut ctx = Context::new();
        let package_name: String = package_name.into();
        ctx.insert("package_name", &package_name.to_uppercase());
        Ok(self.tera.render("lib.rs", &ctx)?)
    }

    pub fn get_intercepted_exports(&self) -> Result<String> {
        let ctx = Context::new();
        Ok(self.tera.render("intercepted_exports.rs", &ctx)?)
    }

    pub fn get_orig_exports(&self, exports: &Vec<String>) -> Result<String> {
        let mut ctx = Context::new();
        let dll_exports: String = exports
            .iter()
            .map(|export_name| {
                format!(
                    r#"load_dll_func(Index_{0}, dll_handle, "{0}");"#,
                    export_name
                )
            })
            .fold(String::new(), |acc, x| acc + "\n    " + &x)
            .trim_start()
            .into();
        ctx.insert("load_dll_exports", &dll_exports);
        Ok(self.tera.render("orig_exports.rs", &ctx)?)
    }

    pub fn get_proxied_exports(
        &self,
        exports: &Vec<String>,
        exclusions: &Vec<String>,
    ) -> Result<String> {
        let mut ctx = Context::new();
        let proxy_exports: String = exports
            .iter()
            .filter(|x| !exclusions.contains(x))
            .map(|export_name| {
                format!(
                    r#"#[no_mangle]
#[naked]
pub unsafe extern "C" fn {0}() {{
    asm!(
        "call wait_init",
        "lea rax, qword ptr [rip + ORIG_FUNCS_PTR]",
        "mov rax, [rax]",
        "add rax, {{orig_index}} * 8",
        "mov rax, [rax]",
        "push rax",
        "ret",
        orig_index = const Index_{0},
        options(noreturn)
    );
}}"#,
                    export_name
                )
            })
            .fold(String::new(), |acc, x| acc + "\n" + &x)
            .trim_start()
            .into();
        ctx.insert("proxy_exports", &proxy_exports);
        Ok(self.tera.render("proxied_exports.rs", &ctx)?)
    }
}

/// Creates a new proxy DLL rust project
pub fn create_proxy_project(
    exports: &Vec<String>,
    dll_name: impl Into<String>,
    out_dir: &PathBuf,
) -> Result<()> {
    let dll_name: String = dll_name.into();
    if out_dir.exists() {
        return Err(anyhow::anyhow!(
            "Folder {} already exists. Aborting",
            out_dir.to_string_lossy()
        ));
    }
    let status = Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(out_dir)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!(
            "Failed to create new proxy project at {}. Exit code: {}",
            out_dir.to_string_lossy(),
            match status.code() {
                Some(code) => format!("{code}"),
                None => "None".into(),
            }
        ));
    }

    let package_name = out_dir
        .file_name()
        .expect("Invalid output directory name")
        .to_str()
        .expect("Unable to detect package name");

    let proxy_gen = ProxyTemplates::new()?;

    let src_cargo_toml = proxy_gen.get_cargo_toml(package_name, dll_name.replace(".dll", ""))?;
    let src_export_indices = proxy_gen.get_export_indices(exports)?;
    let src_intercepted_exports = proxy_gen.get_intercepted_exports()?;
    let src_lib = proxy_gen.get_lib(package_name)?;
    let src_orig_exports = proxy_gen.get_orig_exports(exports)?;
    let src_proxied_exports = proxy_gen.get_proxied_exports(exports, &vec![])?;

    std::fs::write(out_dir.join("Cargo.toml"), src_cargo_toml)?;
    std::fs::write(
        out_dir.join("src").join("export_indices.rs"),
        src_export_indices,
    )?;
    std::fs::write(
        out_dir.join("src").join("intercepted_exports.rs"),
        src_intercepted_exports,
    )?;
    std::fs::write(out_dir.join("src").join("lib.rs"), src_lib)?;
    std::fs::write(
        out_dir.join("src").join("orig_exports.rs"),
        src_orig_exports,
    )?;
    std::fs::write(
        out_dir.join("src").join("proxied_exports.rs"),
        src_proxied_exports,
    )?;

    println!(
        "Successfully created new DLL proxy project '{}' at {}",
        package_name,
        out_dir.to_string_lossy()
    );
    Ok(())
}

/// Updates an existing proxy DLL rust project
pub fn update_proxy_project(exports: &Vec<String>, out_dir: &PathBuf) -> Result<()> {
    if !out_dir.exists() {
        return Err(anyhow::anyhow!(
            "Folder {} doesn't exist. Consider creating a new proxy project instead. Aborting",
            out_dir.to_string_lossy()
        ));
    }

    let package_name = out_dir
        .file_name()
        .expect("Invalid output directory name")
        .to_str()
        .expect("Unable to detect package name");

    // Existing intercepted exports
    let mut intercepted_exports: Vec<String> = Vec::new();
    // All exports, ie. existing ones + new ones
    let mut all_exports: Vec<String> = Vec::new();

    // Get intercepted exports from src/intercepted_exports.rs
    let exports_re = Regex::new(r"pub extern .* fn (.+)\(")?;
    if out_dir.join("src").join("intercepted_exports.rs").exists() {
        for line in
            std::fs::read_to_string(out_dir.join("src").join("intercepted_exports.rs"))?.lines()
        {
            let captures = exports_re.captures(line);
            if let Some(captures) = captures {
                let export_name = captures.get(1).unwrap().as_str().trim();
                intercepted_exports.push(export_name.into());
                println!("Detected intercepted export: {}", export_name);
            }
        }
    }

    // Get existing exports from src/export_indices.rs
    let exports_index_re = Regex::new(r"pub const Index_(.+): usize =")?;
    if out_dir.join("src").join("export_indices.rs").exists() {
        for line in std::fs::read_to_string(out_dir.join("src").join("export_indices.rs"))?.lines()
        {
            let captures = exports_index_re.captures(line);
            if let Some(captures) = captures {
                let export_name = captures.get(1).unwrap().as_str().trim();
                all_exports.push(export_name.into());
            }
        }
    }

    // Add on the new exports that aren't already in all exports
    for export in exports.iter() {
        if !all_exports.contains(export) {
            all_exports.push(export.trim().into());
        }
    }

    // And finally, make sure the exports are sorted
    all_exports.sort();

    let proxy_gen = ProxyTemplates::new()?;

    let src_export_indices = proxy_gen.get_export_indices(&all_exports)?;
    let src_orig_exports = proxy_gen.get_orig_exports(&all_exports)?;
    let src_proxied_exports = proxy_gen.get_proxied_exports(&all_exports, &intercepted_exports)?;

    std::fs::write(
        out_dir.join("src").join("export_indices.rs"),
        src_export_indices,
    )?;
    std::fs::write(
        out_dir.join("src").join("orig_exports.rs"),
        src_orig_exports,
    )?;
    std::fs::write(
        out_dir.join("src").join("proxied_exports.rs"),
        src_proxied_exports,
    )?;

    println!(
        "Successfully updated DLL proxy project '{}' at {}",
        package_name,
        out_dir.to_string_lossy()
    );
    Ok(())
}
