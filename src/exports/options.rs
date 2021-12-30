use std::ffi::c_void;

use fnlog::fn_debug;

use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_OPTIONS_Open"]
pub fn uplay_options_open(_overlapped: *mut UplayOverlapped) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OPTIONS_Enumerate"]
pub fn uplay_options_enumerate(
    _file_handle: u32,
    _key_value_list: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OPTIONS_Set"]
pub fn uplay_options_set(_key_value_list: *const c_void, _key: usize, _value: usize) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OPTIONS_Apply"]
pub fn uplay_options_apply(
    _file_handle: u32,
    _key_value_list: *const c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OPTIONS_ReleaseKeyValueList"]
pub fn uplay_options_release_key_value_list(_key_value_list: *const c_void) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OPTIONS_Close"]
pub fn uplay_options_close(_file_handle: u32) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OPTIONS_SetInGameState"]
pub fn uplay_options_set_in_game_state(_flags: u32) -> usize {
    fn_debug!("__CALL__");
    return 1;
}
