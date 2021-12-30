use std::ffi::CString;
use std::iter;

use anyhow::Result;
use detour::{Error as DetourError, RawDetour};
use log::{error, info, warn};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};

use crate::consts::UPLAY_R1_ORIGINAL_DLL_NAME;
use crate::exports::ach::*;
use crate::exports::avatar::*;
use crate::exports::common::*;
use crate::exports::friends::*;
use crate::exports::installer::*;
use crate::exports::metadata::*;
use crate::exports::options::*;
use crate::exports::overlay::*;
use crate::exports::party::*;
use crate::exports::presence::*;
use crate::exports::save::*;
use crate::exports::user::*;
use crate::exports::win::*;

struct DetourPtr<T>(pub *const T);

unsafe impl<T> Send for DetourPtr<T> {}
unsafe impl<T> Sync for DetourPtr<T> {}

static HOOKS: &'static [(&str, DetourPtr<()>)] = include!(concat!(env!("OUT_DIR"), "/hooks.rs"));

#[inline]
unsafe fn get_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = module
        .encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>();
    let symbol = CString::new(symbol).unwrap();
    let handle = GetModuleHandleW(module.as_ptr());

    return match GetProcAddress(handle, symbol.as_ptr()) as usize {
        0 => None,
        n => Some(n),
    };
}

#[inline]
pub unsafe fn init_hooks() {
    for (name, address) in HOOKS {
        match get_symbol_address(UPLAY_R1_ORIGINAL_DLL_NAME, name) {
            Some(org_address) => {
                info!("Try to install hook: {}", &name);

                if let Err(err) = || -> Result<(), DetourError> {
                    let detour = Box::leak(Box::new(RawDetour::new(
                        org_address as *const (),
                        address.0,
                    )?));
                    detour.enable()?;
                    Ok(())
                }() {
                    error!("{}", err)
                }
            }
            None => warn!("Function not found: {}", name),
        }
    }
}
