use std::ffi::c_char;

#[link(name = "sqexPatch")]
unsafe extern "C" {
    /// 90000 seems to be a common error code?

    pub fn sqexGetPatchTypeName(param_1: u32, param_2: i32, param_3: u32,param_4: u32);

    pub fn sqexPatchDownloadCancel(param_1: i32) -> u32;
    pub fn sqexPatchDownloadGetResponse(param_1: i32, param_2: u32, param_3: u32) -> u32;
    pub fn sqexPatchDownloadIsBusy(param_1: i32, param_2: *mut i32) -> bool;
    pub fn sqexPatchDownloadStart(param_1: i32, param_2: *mut u32, param_3: u32);

    pub fn sqexPatchFinalize(param_1: i32) -> u32;
    pub fn sqexPatchGetErrorCodes(param_1: *mut u32, param_2: i32) -> i32;
    pub fn sqexPatchGetFileHashString(param_1: u32, param_2: u32, param_3: u32);
    pub fn sqexPatchGetNumOfPatch(repo: i32) -> *mut i32;
    pub fn sqexPatchGetPatchElement(param_1: i32, param_2: *mut i32, param_3: i32) -> u32;

    /// e.x. "2.00"
    pub fn sqexPatchGetVersion() -> *const c_char;
    pub fn sqexPatchGetVersionName(param_1: u32, param_2: u32, param_3: i32, param_4: u32);

    /// type_name must be either "boot" or "game", otherwise returns -1
    /// hashes is the boot hash/session id like "ffxivboot.exe/149504/5f2a70612aa58378eb347869e75adeb8f5581a1b". Fun fact, the Chinese version literally hardcodes this, so it's just some random person's SID?!?!
    /// Can return either 1 (success?) or -1 (fail)
    pub fn sqexPatchInitialize(type_name: *const c_char, hashes: *const c_char) -> i32;

    pub fn sqexPatchInstallCancel(param_1: i32) -> u32;
    pub fn sqexPatchInstallGetResponse(param_1: i32) -> u32;
    pub fn sqexPatchInstallIsBusy(param_1: i32, param_2: u32) -> bool;
    pub fn sqexPatchInstallStart(param_1: i32, param_2: u32, param_3: u32) -> u32;

    pub fn sqexPatchSetVersionName(param_1: u32, param_2: u32, param_3: *mut c_char);

    /// Cancels the request, 0 if successful and 0x15f90 if not.
    pub fn sqexPatchVersionCheckCancel(repo: i32) -> u32;

    /// Returns 0x15f90 if the repo is invalid, returns 0 for... everything else (yes, really.)
    pub fn sqexPatchVersionCheckGetResponse(repo: i32) -> i32;

    /// Returns true if we're still contacting the patch server for this repo.
    pub fn sqexPatchVersionCheckIsBusy(repo: i32) -> bool;

    /// Note: Repos start at 1

    /// host_header is NOT the url, that's the "Host" header!
    /// Contacts the patch server, for a version check I guess?
    /// Note: I'm 99% sure this happens in another thread, so you need to keep checking IsBusy.
    pub fn sqexPatchVersionCheckStart(repo: i32, host: *const c_char, param_3: i32, host_header: *const c_char, param_5: *const c_char, platform: *const c_char, channel: *const c_char) -> i32;

    pub fn sqexPatchCheckStartEx(param_1: i32, param_2: u32, param_3: i32, param_4: u32, param_5: u32, param_6: u32, param_7: u32, param_8: *mut u32, param_9: u32);
}
