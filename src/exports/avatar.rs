use std::ffi::c_void;
use std::os::raw::c_char;

use fnlog::fn_debug;

use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_AVATAR_GetBitmap"]
pub fn uplay_avatar_get_bitmap(
    _avatar_id: *const u32,
    _avatar_size: u32,
    _rgba: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_AVATAR_GetAvatarIdForCurrentUser"]
pub fn uplay_avatar_get_avatar_id_for_current_user(
    _avatar_id: *mut isize,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_AVATAR_Get"]
pub fn uplay_avatar_get(
    _account_id_utf8: *const c_char,
    _avatar_size: u32,
    _rgba: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}
