use std::ffi::c_void;

use fnlog::fn_debug;

#[export_name = "UPLAY_PRESENCE_SetPresence"]
pub fn uplay_presence_set_presence(_presence_id: u32, _tokens: *const c_void) -> usize {
    fn_debug!("__CALL__");
    return 1;
}
