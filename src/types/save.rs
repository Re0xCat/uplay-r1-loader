use std::fmt::Debug;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct UplaySave {
    pub slot_id: u32,
    pub name: *const c_char,
}
