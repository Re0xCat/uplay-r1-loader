use std::ffi::c_void;

use fnlog::fn_debug;

use crate::types::list::UplayList;
use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_WIN_GetRewards"]
pub fn uplay_win_get_rewards(_reward_list: *mut *mut UplayList, _overlapped: *mut UplayOverlapped) {
    fn_debug!("__CALL__");
}

#[export_name = "UPLAY_WIN_RefreshActions"]
pub fn uplay_win_refresh_actions() -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_WIN_SetActionsCompleted"]
pub fn uplay_win_set_actions_completed(
    _action_ids_utf8: *const c_void,
    _action_ids_count: isize,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 1;
}
