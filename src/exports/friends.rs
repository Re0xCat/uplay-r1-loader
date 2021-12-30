use std::ffi::c_void;
use std::os::raw::c_char;

use fnlog::fn_debug;

use crate::types::list::UplayList;
use crate::types::overlapped::UplayOverlapped;

#[export_name = "UPLAY_FRIENDS_Init"]
pub fn uplay_friends_init(_flags: u32) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_GetFriendList"]
pub fn uplay_friends_get_friend_list(
    _friend_list_filter: u32,
    _friend_list: *mut *mut UplayList,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_RequestFriendship"]
pub fn uplay_friends_request_friendship(
    _search_string_utf88: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_IsFriend"]
pub fn uplay_friends_is_friend(_account_id_utf8: *const c_char) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_AddToBlackList"]
pub fn uplay_friends_add_to_black_list(
    _account_id_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_IsBlackListed"]
pub fn uplay_friends_is_black_listed(_account_id_utf8: *const c_char) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_ShowFriendSelectionUI"]
pub fn uplay_friends_show_friend_selection_ui(
    _account_id_filter_list_utf8: *const c_char,
    _account_id_filter_list_length: u32,
    _overlapped: *mut UplayOverlapped,
    _result: *mut c_void,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_EnableFriendMenuItem"]
pub fn uplay_friends_enable_friend_menu_item(
    _account_id_filter_list_utf8: *const c_char,
    _account_id_filter_list_length: u32,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_DisableFriendMenuItem"]
pub fn uplay_friends_disable_friend_menu_item(_id: usize) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_InviteToGame"]
pub fn uplay_friends_invite_to_game(
    _account_id_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}
