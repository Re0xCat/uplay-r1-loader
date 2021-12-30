use std::ffi::c_void;
use std::os::raw::c_char;

use fnlog::fn_debug;

use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_OVERLAY_Show"]
pub fn uplay_overlay_show(_overlay_section: i32, _overlapped: *const UplayOverlapped) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OVERLAY_SetShopUrl"]
pub unsafe fn uplay_overlay_set_shop_url(
    _url_utf8: *const c_char,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");

    if !overlapped.is_null() {
        (*overlapped).is_completed = 1;
        (*overlapped).reserved = 0;
    }

    return 1;
}

#[export_name = "UPLAY_OVERLAY_ShowShopUrl"]
pub fn uplay_overlay_show_shop_url(_url_utf8: *const c_void) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_OVERLAY_ShowNotification"]
pub fn uplay_overlay_show_notification(_notification_id: *const c_void) -> usize {
    fn_debug!("__CALL__");
    return 0;
}
