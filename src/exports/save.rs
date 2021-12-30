use std::ffi::CStr;
use std::fs::OpenOptions;
use std::os::raw::c_char;
use std::{ptr, slice};

use anyhow::Result;
use fnlog::fn_debug;
use log::error;
use ustr::Ustr;

use crate::global::SAVES_OPEN_OPTIONS;
use crate::helpers::alloc::alloc;
use crate::helpers::manifest::{get_manifest_path, read_manifest, write_manifest};
use crate::helpers::save::{get_saves, read_save, remove_save, write_save};
use crate::models::manifest::{Manifest, Save};
use crate::types::list::{List, UplayList};
use crate::types::overlapped::UplayOverlapped;
use crate::types::save::UplaySave;

#[export_name = "UPLAY_SAVE_GetSavegames"]
pub unsafe fn uplay_save_get_savegames(
    games_list: *mut *mut UplayList,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!(
        "SaveGameList: {:?} Overlapped: {:?}",
        games_list,
        overlapped
    );

    let result = (|| -> Result<()> {
        let saves = get_saves()?;
        let saves_size = saves.len();

        let uplay_saves = alloc(Vec::new());

        for (slot_id, name, _) in saves {
            let name_c_str = Ustr::from(&name).as_char_ptr();
            let uplay_save = alloc(UplaySave {
                slot_id,
                name: name_c_str,
            });

            (*uplay_saves).push(uplay_save)
        }

        let list = alloc(UplayList {
            count: saves_size as u32,
            list: List {
                saves: (*uplay_saves).as_ptr() as *const *const UplaySave,
            },
        });

        *games_list = list;

        Ok(())
    })();

    match result {
        Ok(_) => {
            if !overlapped.is_null() {
                (*overlapped).set_result();
            }

            return 1;
        }
        Err(err) => error!("{}", err),
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_Open"]
pub unsafe fn uplay_save_open(
    slot_id: u32,
    mode: u32,
    save_handle: *mut u32,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!(
        "SlotId: {} Mode: {} SaveHandle: {:?}, Overlapped: {:?}",
        slot_id,
        mode,
        save_handle,
        overlapped
    );

    let options = if mode == 1 {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .to_owned()
    } else {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .to_owned()
    };

    SAVES_OPEN_OPTIONS.write().insert(slot_id, options);

    *save_handle = slot_id;

    if !overlapped.is_null() {
        (*overlapped).set_result();
    }

    return 1;
}

#[export_name = "UPLAY_SAVE_Close"]
pub unsafe fn uplay_save_close(slot_id: u32) -> usize {
    fn_debug!("__CALL__");

    SAVES_OPEN_OPTIONS.write().remove(&slot_id);
    return 1;
}

#[export_name = "UPLAY_SAVE_Read"]
pub unsafe fn uplay_save_read(
    slot_id: u32,
    num_of_bytes_to_read: u32,
    offset: u32,
    buffer: *mut *mut c_char,
    num_of_bytes_read: *mut usize,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");

    let result = (|| -> Result<()> {
        let (data, size) = read_save(slot_id, num_of_bytes_to_read, offset)?;
        ptr::copy(data.as_ptr() as *const c_char, *buffer, data.len());

        *num_of_bytes_read = size;
        Ok(())
    })();

    match result {
        Ok(_) => {
            if !overlapped.is_null() {
                (*overlapped).set_result();
            }

            return 1;
        }
        Err(err) => error!("{}", err),
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_Write"]
pub unsafe fn uplay_save_write(
    slot_id: u32,
    num_of_bytes_to_write: u32,
    buffer: *const *const c_char,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!(
        "SlotId: {} NumOfBytesToWrite: {} Buffer: {:?} Overlapped: {:?}",
        slot_id,
        num_of_bytes_to_write,
        buffer,
        overlapped
    );

    if let Some(open_options) = SAVES_OPEN_OPTIONS.read().get(&slot_id) {
        let result = (|| -> Result<()> {
            let buffer =
                slice::from_raw_parts(*buffer as *const u8, num_of_bytes_to_write as usize);
            let _ = write_save(slot_id, open_options, buffer)?;
            Ok(())
        })();

        match result {
            Ok(_) => {
                if !overlapped.is_null() {
                    (*overlapped).set_result();
                }

                return 1;
            }
            Err(err) => error!("{}", err),
        }
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_SetName"]
pub unsafe fn uplay_save_set_name(save_handle: u32, name_utf8: *const c_char) -> usize {
    fn_debug!("SaveHandle: {} NameUtf8: {:?}", save_handle, name_utf8);

    let result = (|| -> Result<()> {
        let mut manifest = if !get_manifest_path()?.exists() {
            Manifest { saves: Vec::new() }
        } else {
            read_manifest()?
        };

        let save_id = save_handle;
        let save_name = CStr::from_ptr(name_utf8).to_str()?.to_string();

        match manifest.saves.iter_mut().find(|save| save.id == save_id) {
            Some(save) => {
                save.name = save_name;
            }
            None => manifest.saves.push(Save {
                id: save_id,
                name: save_name,
            }),
        }

        write_manifest(&manifest)?;
        Ok(())
    })();

    match result {
        Ok(_) => return 1,
        Err(err) => error!("{}", err),
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_Remove"]
pub unsafe fn uplay_save_remove(slot_id: u32, overlapped: *mut UplayOverlapped) -> usize {
    fn_debug!("__CALL__");

    let result = (|| -> Result<()> {
        remove_save(slot_id)?;
        Ok(())
    })();

    match result {
        Ok(_) => {
            if !overlapped.is_null() {
                (*overlapped).set_result();
            }

            return 1;
        }
        Err(err) => error!("{}", err),
    }

    return 0;
}
