/* automatically generated by rust-bindgen 0.70.1, minified by cargo-minify */

pub type pam_handle_t = u8;
pub type _bindgen_ty_1 = libc::c_uint;
pub type _bindgen_ty_2 = libc::c_uint;
pub type _bindgen_ty_3 = libc::c_int;
pub type _bindgen_ty_4 = libc::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pam_message {
    pub msg_style: libc::c_int,
    pub msg: *mut libc::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pam_response {
    pub resp: *mut libc::c_char,
    pub resp_retcode: libc::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pam_conv {
    pub conv: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: libc::c_int,
            arg2: *mut *const pam_message,
            arg3: *mut *mut pam_response,
            arg4: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub appdata_ptr: *mut libc::c_void,
}
pub const PAM_SUCCESS: _bindgen_ty_1 = 0;
pub const PAM_OPEN_ERR: _bindgen_ty_1 = 1;
pub const PAM_SYMBOL_ERR: _bindgen_ty_1 = 2;
pub const PAM_SERVICE_ERR: _bindgen_ty_1 = 3;
pub const PAM_SYSTEM_ERR: _bindgen_ty_1 = 4;
pub const PAM_BUF_ERR: _bindgen_ty_1 = 5;
pub const PAM_CONV_ERR: _bindgen_ty_1 = 6;
pub const PAM_PERM_DENIED: _bindgen_ty_1 = 7;
pub const PAM_MAXTRIES: _bindgen_ty_1 = 8;
pub const PAM_AUTH_ERR: _bindgen_ty_1 = 9;
pub const PAM_NEW_AUTHTOK_REQD: _bindgen_ty_1 = 10;
pub const PAM_CRED_INSUFFICIENT: _bindgen_ty_1 = 11;
pub const PAM_AUTHINFO_UNAVAIL: _bindgen_ty_1 = 12;
pub const PAM_USER_UNKNOWN: _bindgen_ty_1 = 13;
pub const PAM_CRED_UNAVAIL: _bindgen_ty_1 = 14;
pub const PAM_CRED_EXPIRED: _bindgen_ty_1 = 15;
pub const PAM_CRED_ERR: _bindgen_ty_1 = 16;
pub const PAM_ACCT_EXPIRED: _bindgen_ty_1 = 17;
pub const PAM_AUTHTOK_EXPIRED: _bindgen_ty_1 = 18;
pub const PAM_SESSION_ERR: _bindgen_ty_1 = 19;
pub const PAM_AUTHTOK_ERR: _bindgen_ty_1 = 20;
pub const PAM_AUTHTOK_RECOVERY_ERR: _bindgen_ty_1 = 21;
pub const PAM_AUTHTOK_LOCK_BUSY: _bindgen_ty_1 = 22;
pub const PAM_AUTHTOK_DISABLE_AGING: _bindgen_ty_1 = 23;
pub const PAM_NO_MODULE_DATA: _bindgen_ty_1 = 24;
pub const PAM_IGNORE: _bindgen_ty_1 = 25;
pub const PAM_ABORT: _bindgen_ty_1 = 26;
pub const PAM_TRY_AGAIN: _bindgen_ty_1 = 27;
pub const PAM_MODULE_UNKNOWN: _bindgen_ty_1 = 28;
pub const PAM_BAD_ITEM: _bindgen_ty_1 = 31;
pub const PAM_PROMPT_ECHO_OFF: _bindgen_ty_2 = 1;
pub const PAM_PROMPT_ECHO_ON: _bindgen_ty_2 = 2;
pub const PAM_ERROR_MSG: _bindgen_ty_2 = 3;
pub const PAM_TEXT_INFO: _bindgen_ty_2 = 4;
pub const PAM_MAX_RESP_SIZE: _bindgen_ty_2 = 512;
pub const PAM_SILENT: _bindgen_ty_3 = -2147483648;
pub const PAM_DISALLOW_NULL_AUTHTOK: _bindgen_ty_3 = 1;
pub const PAM_REINITIALIZE_CRED: _bindgen_ty_3 = 4;
pub const PAM_CHANGE_EXPIRED_AUTHTOK: _bindgen_ty_3 = 4;
pub const PAM_USER: _bindgen_ty_4 = 2;
pub const PAM_TTY: _bindgen_ty_4 = 3;
pub const PAM_RUSER: _bindgen_ty_4 = 8;
extern "C" {
    pub fn pam_acct_mgmt(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_authenticate(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_chauthtok(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_close_session(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_end(_pamh: *mut pam_handle_t, _status: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_get_item(
        _pamh: *const pam_handle_t,
        _item_type: libc::c_int,
        _item: *mut *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_getenvlist(_pamh: *mut pam_handle_t) -> *mut *mut libc::c_char;
}
extern "C" {
    pub fn pam_open_session(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_set_item(
        _pamh: *mut pam_handle_t,
        _item_type: libc::c_int,
        _item: *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_setcred(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_start(
        _service: *const libc::c_char,
        _user: *const libc::c_char,
        _pam_conv: *const pam_conv,
        _pamh: *mut *mut pam_handle_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_strerror(
        _pamh: *const pam_handle_t,
        _error_number: libc::c_int,
    ) -> *const libc::c_char;
}
