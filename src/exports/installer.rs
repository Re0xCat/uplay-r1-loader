use std::ffi::c_void;
use std::os::raw::c_char;

use fnlog::fn_debug;
use once_cell::sync::Lazy;
use ustr::Ustr;

use crate::global::CONFIG;
use crate::types::list::UplayList;

static LANGUAGE: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.uplay.language));

#[export_name = "UPLAY_INSTALLER_Init"]
pub fn uplay_installer_init(_flags: u32) -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_UpdateInstallOrder"]
pub fn uplay_installer_update_install_order(_chunk_ids: *const c_void, _chunk_count: u32) -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_AreChunksInstalled"]
pub fn uplay_installer_are_chunks_installed(chunk_ids: *const c_void, _chunk_count: u32) -> usize {
    fn_debug!("__CALL__");
    return !chunk_ids.is_null() as usize;
}

#[export_name = "UPLAY_INSTALLER_GetChunks"]
pub fn uplay_installer_get_chunks(_chunk_id_list: *mut *mut UplayList) -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_GetChunkIdsFromTag"]
pub fn uplay_installer_get_chunk_ids_from_tag(
    _tag_utf8: *const c_char,
    _chunk_id_list: *mut *mut UplayList,
) -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_GetLanguageUtf8"]
pub fn uplay_installer_get_language_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return LANGUAGE.as_char_ptr();
}
