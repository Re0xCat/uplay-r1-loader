use std::ffi::c_void;
use std::os::raw::c_char;

use fnlog::fn_debug;

use crate::types::list::UplayList;
use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_Start"]
pub fn uplay_start(_uplay_id: u32, _flags: u32) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_Startup"]
pub fn uplay_startup(
    _uplay_id: u32,
    _game_version: u32,
    _language_country_code_utf8: *const c_char,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_Quit"]
pub fn uplay_quit() -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_HasOverlappedOperationCompleted"]
pub fn uplay_has_overlapped_operation_completed(overlapped: *const UplayOverlapped) -> usize {
    fn_debug!("__CALL__");

    unsafe {
        if !overlapped.is_null() {
            return ((*overlapped).is_completed != 0) as usize;
        }
    }

    return 0;
}

#[export_name = "UPLAY_Init"]
pub fn uplay_init() -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_GetOverlappedOperationResult"]
pub fn uplay_get_overlapped_operation_result(
    overlapped: *const UplayOverlapped,
    result: *mut c_void,
) -> usize {
    fn_debug!("__CALL__");

    unsafe {
        if !overlapped.is_null() && (*overlapped).is_completed == 1 {
            *(result as *mut i32) = (*overlapped).reserved;
            return 1;
        }
    }

    return 0;
}

#[export_name = "UPLAY_Update"]
pub fn uplay_update() -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_Release"]
pub fn uplay_release(list: *mut UplayList) -> usize {
    fn_debug!("__CALL__");

    if !list.is_null() {
        unsafe {
            Box::from_raw(list);
        }
    }

    return 1;
}

#[export_name = "UPLAY_GetNextEvent"]
pub fn uplay_get_next_event(_: *mut isize) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_GetLastError"]
pub fn uplay_get_last_error(_error_string: *const c_char) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_GetInstallationError"]
pub fn uplay_get_installation_error(_: *const c_char) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_SetGameSession"]
pub fn uplay_set_game_session(
    _game_session_identifier: isize,
    _session_data: isize,
    _flags: u32,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_ClearGameSession"]
pub fn uplay_clear_game_session() {
    fn_debug!("__CALL__");
}
