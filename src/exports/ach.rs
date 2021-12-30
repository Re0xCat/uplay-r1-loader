use std::ffi::c_void;
use std::os::raw::c_char;

use fnlog::fn_debug;

use crate::types::list::UplayList;
use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_ACH_GetAchievementImage"]
pub fn uplay_ach_get_achievement_image(
    _achievement_id: u32,
    _image: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_ACH_EarnAchievement"]
pub fn uplay_ach_earn_achievement(
    _achievement_id: u32,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_ACH_Write"]
pub fn uplay_ach_write(_achievement: *const c_char) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_ACH_GetAchievements"]
pub fn uplay_ach_get_achievements(
    _filter: u32,
    _account_id_utf8_or_null_if_current_user: *const c_char,
    _achievement_list: *mut *mut UplayList,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_ACH_ReleaseAchievementList"]
pub fn uplay_ach_release_achievement_list(_list: *mut *mut UplayList) -> usize {
    fn_debug!("__CALL__");
    return 1;
}
