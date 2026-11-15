use crate::zlong;

use core::ffi::{c_char, c_int, c_short, c_uchar};

/*********************************************/
/* Structure to save and restore hist module */
/*********************************************/
#[repr(C)]
#[derive(Default)]
pub struct hist_stack {
    pub histactive: c_int,
    pub histdone: c_int,
    pub stophist: c_int,
    pub hlinesz: c_int,
    pub defev: zlong,
    pub hline: *mut c_char,
    pub hptr: *mut c_char,
    pub chwords: *mut c_short,
    pub chwordlen: c_int,
    pub chwordpos: c_int,
    pub hgetc: Option<unsafe extern "C" fn() -> c_int>,
    pub hungetc: Option<unsafe extern "C" fn(c_int) -> ()>,
    pub hwaddc: Option<unsafe extern "C" fn(c_int) -> ()>,
    pub hwbegin: Option<unsafe extern "C" fn(c_int) -> ()>,
    pub hwabort: Option<unsafe extern "C" fn() -> ()>,
    pub hwend: Option<unsafe extern "C" fn() -> ()>,
    pub addtoline: Option<unsafe extern "C" fn(c_int) -> ()>,
    pub cstack: *mut c_uchar,
    pub csp: c_int,
    pub hist_keep_comment: c_int,
}
