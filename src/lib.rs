use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use rayon::prelude::*;
use reqwest;
use std::collections::HashMap;

fn sum_of_squares() -> i32 {
    (0..100000).into_par_iter().map(|x| x * 2).sum()
}

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    let n = sum_of_squares();
    let s = format!("{}", n);
    let resp: HashMap<String, String> = reqwest::get("https://www.csdn.net").unwrap()
        .json().unwrap();
    let s1 = format!("{:#?}", resp);
    CString::new(s1 + recipient)
        .unwrap()
        .into_raw()
}


#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        CString::from_raw(s)
    };
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use super::*;
    use jni;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;

    #[no_mangle]
    pub unsafe extern fn Java_com_example_Greetings_greeting(env: JNIEnv, _cls: JClass,
                                                             java_pattern: JString) -> jstring {
        let world = rust_greeting(
            env.get_string(java_pattern).unwrap().as_ptr()
        );

        let output = env.new_string(CStr::from_ptr(world).to_str().unwrap()).unwrap();
        rust_greeting_free(world);

        output.into_inner()
    }
}