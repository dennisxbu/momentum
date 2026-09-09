use crate::domain::{MomentumError, Result};
use std::{ffi::c_void, fs, path::Path};

const KEY_SIZE: usize = 32;

pub fn load_or_create_key(path: &Path) -> Result<Vec<u8>> {
    if path.exists() {
        let protected = fs::read(path)?;
        return unprotect(&protected);
    }

    let mut key = vec![0_u8; KEY_SIZE];
    getrandom::fill(&mut key).map_err(|error| {
        MomentumError::Security(format!(
            "Zufälliger Schlüssel konnte nicht erzeugt werden: {error}"
        ))
    })?;
    let protected = protect(&key)?;
    fs::write(path, protected)?;
    Ok(key)
}

#[cfg(windows)]
#[repr(C)]
struct DataBlob {
    cb_data: u32,
    pb_data: *mut u8,
}

#[cfg(windows)]
#[link(name = "Crypt32")]
unsafe extern "system" {
    fn CryptProtectData(
        input: *mut DataBlob,
        description: *const u16,
        entropy: *mut DataBlob,
        reserved: *mut c_void,
        prompt: *mut c_void,
        flags: u32,
        output: *mut DataBlob,
    ) -> i32;
    fn CryptUnprotectData(
        input: *mut DataBlob,
        description: *mut *mut u16,
        entropy: *mut DataBlob,
        reserved: *mut c_void,
        prompt: *mut c_void,
        flags: u32,
        output: *mut DataBlob,
    ) -> i32;
}

#[cfg(windows)]
#[link(name = "Kernel32")]
unsafe extern "system" {
    fn LocalFree(memory: *mut c_void) -> *mut c_void;
}

#[cfg(windows)]
fn protect(value: &[u8]) -> Result<Vec<u8>> {
    const CRYPTPROTECT_UI_FORBIDDEN: u32 = 0x1;
    let mut input = DataBlob {
        cb_data: value.len() as u32,
        pb_data: value.as_ptr() as *mut u8,
    };
    let mut output = DataBlob {
        cb_data: 0,
        pb_data: std::ptr::null_mut(),
    };
    let mut description: Vec<u16> = "Momentum local database key\0".encode_utf16().collect();
    let succeeded = unsafe {
        CryptProtectData(
            &mut input,
            description.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return Err(MomentumError::Security(format!(
            "Windows-Datenschutz konnte den Schlüssel nicht schützen: {}",
            std::io::Error::last_os_error()
        )));
    }
    let result =
        unsafe { std::slice::from_raw_parts(output.pb_data, output.cb_data as usize).to_vec() };
    unsafe { LocalFree(output.pb_data.cast()) };
    Ok(result)
}

#[cfg(windows)]
fn unprotect(value: &[u8]) -> Result<Vec<u8>> {
    const CRYPTPROTECT_UI_FORBIDDEN: u32 = 0x1;
    let mut input = DataBlob {
        cb_data: value.len() as u32,
        pb_data: value.as_ptr() as *mut u8,
    };
    let mut output = DataBlob {
        cb_data: 0,
        pb_data: std::ptr::null_mut(),
    };
    let mut description: *mut u16 = std::ptr::null_mut();
    let succeeded = unsafe {
        CryptUnprotectData(
            &mut input,
            &mut description,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return Err(MomentumError::Security(format!(
            "Windows-Datenschutz konnte den Schlüssel nicht öffnen: {}",
            std::io::Error::last_os_error()
        )));
    }
    let result =
        unsafe { std::slice::from_raw_parts(output.pb_data, output.cb_data as usize).to_vec() };
    unsafe {
        LocalFree(output.pb_data.cast());
        if !description.is_null() {
            LocalFree(description.cast());
        }
    }
    if result.len() != KEY_SIZE {
        return Err(MomentumError::Security(
            "Der geschützte Datenbankschlüssel hat eine ungültige Länge.".into(),
        ));
    }
    Ok(result)
}

#[cfg(not(windows))]
fn protect(_: &[u8]) -> Result<Vec<u8>> {
    Err(MomentumError::Security(
        "DPAPI ist nur unter Windows verfügbar.".into(),
    ))
}

#[cfg(not(windows))]
fn unprotect(_: &[u8]) -> Result<Vec<u8>> {
    Err(MomentumError::Security(
        "DPAPI ist nur unter Windows verfügbar.".into(),
    ))
}
