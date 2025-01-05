extern crate jni;

use std::ffi::c_char;

use jni::sys::jstring;
use jni::JNIEnv;

use jni::objects::{JClass, JString};

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_cc_kt_sf_MainActivity_00024Companion_getSize(
    _env: JNIEnv,
    _class: JClass,
    _a: i32,
) -> i32 {
    process(_a)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_cc_kt_sf_MainActivity_00024Companion_generateKey(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let ss = jstring_to_rust_string(&mut env, input).unwrap();
    let rs = reverse(&ss);
    let js = rust_string_to_jstring(&mut env, &rs).unwrap();
    js
}

fn process(a: i32) -> i32 {
    let b = a * a;
    b
}

fn reverse(input: &str) -> String {
    let new_string = input.chars().rev().collect();
    new_string
}

fn jstring_to_rust_string(env: &mut JNIEnv, java_string: JString) -> Option<String> {
    env.get_string(&java_string).ok().map(|s| s.into())
}

fn rust_string_to_jstring(env: &mut JNIEnv, rust_string: &str) -> Option<jni::sys::jstring> {
    env.new_string(rust_string)
        .ok()
        .map(|jstring| jstring.into_raw())
}
