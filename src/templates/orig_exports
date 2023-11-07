use crate::export_indices::*;
use crate::{ORIGINAL_FUNCS, ORIG_DLL_HANDLE};
use std::ffi::CString;
use winapi::{
    shared::minwindef::{FARPROC, HMODULE},
    um::libloaderapi::GetProcAddress,
};

/// Loads up the address of the original function in the given module
unsafe fn load_dll_func(index: usize, h_module: HMODULE, func: &str) {
    let func_c_string = CString::new(func).unwrap();
    let proc_address: FARPROC = GetProcAddress(h_module, func_c_string.as_ptr());
    ORIGINAL_FUNCS[index] = proc_address;
    println!("[0x{:016x}] Loaded {}", proc_address as u64, func);
}

/// Loads the original DLL functions for later use
pub unsafe fn load_dll_funcs() {
    println!("Loading original DLL functions");
    if ORIG_DLL_HANDLE.is_none() {
        eprintln!("Original DLL handle is none. Cannot load original DLL funcs");
        return;
    }
    let dll_handle = ORIG_DLL_HANDLE.unwrap();
    {{ load_dll_exports }}
}
