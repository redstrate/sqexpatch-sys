use std::ffi::{c_char, CStr, CString};

use sqexpatch_sys::*;

fn main() {
    unsafe {
        println!("sqexPatchGetVersion = {}", CStr::from_ptr(sqexPatchGetVersion() as *const i8).to_str().unwrap());

        let string1 = CString::new("boot").unwrap();
        let string2 = CString::new("ffxivboot.exe/149504/5f2a70612aa58378eb347869e75adeb8f5581a1b").unwrap();
        let res = sqexPatchInitialize(string1.as_ptr() as *const c_char, string2.as_ptr() as *const c_char);
        println!("sqexPatchInitialize = {res}");

        let empty = CString::new("win32").unwrap();
        let host = CString::new("patch-bootver.ffxiv.localhost").unwrap();
        let channel = CString::new("1.0").unwrap();
        let platform = CString::new("win32_neo_game").unwrap();
        let res = sqexPatchVersionCheckStart(1, host.as_ptr() as *const c_char, 6900, host.as_ptr() as *const c_char, empty.as_ptr() as *const c_char, platform.as_ptr() as *const c_char, channel.as_ptr() as *const c_char);
        println!("sqexPatchVersionCheckStart = {res}");

        let res = sqexPatchVersionCheckIsBusy(1);
        println!("sqexPatchVersionCheckIsBusy = {res}");

        while sqexPatchVersionCheckIsBusy(1) {
            //println!("Waiting...");
        }

        let res = sqexPatchVersionCheckGetResponse(1);
        println!("sqexPatchVersionCheckGetResponse = {res}");

        let res = sqexPatchGetNumOfPatch(1);
        println!("sqexPatchGetNumOfPatch = {:#?}", res);
    }
}
