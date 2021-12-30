use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;

use fnlog::fn_debug;
use once_cell::sync::Lazy;
use ustr::Ustr;

use crate::global::CONFIG;
use crate::helpers::alloc::alloc;
use crate::types::key::UplayKey;
use crate::types::list::{List, UplayList};
use crate::types::overlapped::UplayOverlapped;

static ACCOUNT_ID: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.uplay.profile.account_id));
static USERNAME: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.uplay.profile.username));
static EMAIL: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.uplay.profile.email));
static PASSWORD: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.uplay.profile.password));
static TICKET: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.uplay.profile.ticket));

#[export_name = "UPLAY_USER_GetCdKeys"]
pub unsafe fn uplay_user_get_cd_keys(
    cd_keys_list: *mut *mut UplayList,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!(
        "CdKeysList: {:?} Overlapped: {:?}",
        cd_keys_list,
        overlapped
    );

    let keys_size = CONFIG.uplay.cd_keys.len();
    let uplay_keys = alloc(Vec::new());

    for key in &CONFIG.uplay.cd_keys {
        let uplay_key = alloc(UplayKey {
            cd_key: Ustr::from(&key).as_char_ptr(),
        });

        (*uplay_keys).push(uplay_key)
    }

    let list = alloc(UplayList {
        count: keys_size as u32,
        list: List {
            keys: (*uplay_keys).as_ptr() as *const *const UplayKey,
        },
    });

    *cd_keys_list = list;

    if !overlapped.is_null() {
        (*overlapped).unk = 0;
        (*overlapped).is_completed = 1;
        (*overlapped).reserved = 0;
    }

    return 1;
}

#[export_name = "UPLAY_USER_ReleaseCdKeyList"]
pub fn uplay_user_release_cd_key_list(cd_key_list: *mut UplayList) -> usize {
    fn_debug!("__CALL__");

    if !cd_key_list.is_null() {
        unsafe {
            Box::from_raw(cd_key_list);
        }
    }

    return 1;
}

#[export_name = "UPLAY_USER_GetCredentials"]
pub unsafe fn uplay_user_get_credentials(
    user_credentials: *mut c_void,
    overlapped: *mut UplayOverlapped,
) -> usize {
    fn_debug!("__CALL__");

    ptr::copy(
        USERNAME.as_ptr() as *const c_void,
        user_credentials,
        USERNAME.as_bytes().len() + 1,
    );

    if !overlapped.is_null() {
        (*overlapped).set_result();
    }

    return 0;
}

#[export_name = "UPLAY_USER_GetCdKeyUtf8"]
pub fn uplay_user_get_cd_key_utf8(_uplay_id: u32) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_GetAccountIdUtf8"]
pub fn uplay_user_get_account_id_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return ACCOUNT_ID.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetUsernameUtf8"]
pub fn uplay_user_get_username_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return USERNAME.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetNameUtf8"]
pub fn uplay_user_get_name_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return USERNAME.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetEmailUtf8"]
pub fn uplay_user_get_email_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return EMAIL.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetPasswordUtf8"]
pub fn uplay_user_get_password_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return PASSWORD.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetAccountId"]
pub fn uplay_user_get_account_id() -> *const c_char {
    fn_debug!("__CALL__");
    return ACCOUNT_ID.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetUsername"]
pub fn uplay_user_get_username() -> *const c_char {
    fn_debug!("__CALL__");
    return USERNAME.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetEmail"]
pub fn uplay_user_get_email() -> *const c_char {
    fn_debug!("__CALL__");
    return EMAIL.as_char_ptr();
}

#[export_name = "UPLAY_USER_GetPassword"]
pub fn uplay_user_get_password() -> *const c_char {
    fn_debug!("__CALL__");
    return PASSWORD.as_char_ptr();
}

#[export_name = "UPLAY_USER_IsConnected"]
pub fn uplay_user_is_connected() -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_USER_IsOwned"]
pub fn uplay_user_is_owned() -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_USER_IsInOfflineMode"]
pub fn uplay_user_is_in_offline_mode() -> usize {
    fn_debug!("__CALL__");
    return CONFIG.uplay.offline_mode as usize;
}

#[export_name = "UPLAY_USER_GetTicketUtf8"]
pub fn uplay_user_get_ticket_utf8() -> *const c_char {
    fn_debug!("__CALL__");
    return TICKET.as_char_ptr();
}

#[export_name = "UPLAY_USER_ConsumeItem"]
pub fn uplay_user_consume_item(
    _transaction_id_utf8: *const c_char,
    _uplay_id: u32,
    _quantity: u32,
    _signature_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
    _result: *mut c_void,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_GetConsumeItem"]
pub fn uplay_user_get_consume_item() -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_GetConsumableItems"]
pub fn uplay_user_get_consumable_items(_consumable_items_list: *mut *mut UplayList) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_ReleaseConsumeItemResult"]
pub fn uplay_user_release_consume_item_result(_consume_item_result: *mut c_void) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_SetGameSession"]
pub fn uplay_user_set_game_session(
    _game_session_identifier: usize,
    _session_data: *const c_void,
    _flags: u32,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_ClearGameSession"]
pub fn uplay_user_clear_game_session() -> usize {
    fn_debug!("__CALL__");
    return 1;
}

#[export_name = "UPLAY_USER_GetGPUScoreConfidenceLevel"]
pub fn uplay_user_get_gpu_score_confidence_level(_confidence_level: usize) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_GetGPUScore"]
pub fn uplay_user_get_gpu_score(_gpu_score: usize) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_USER_GetCPUScore"]
pub fn uplay_user_get_cpu_score(_cpu_score: usize) -> usize {
    fn_debug!("__CALL__");
    return 0;
}
