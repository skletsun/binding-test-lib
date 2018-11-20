#[macro_use]
extern crate lazy_static;
extern crate jni;
extern crate second;
extern crate libloading;
use self::libloading::{Library, Symbol};

use jni::{
    objects::JClass,
    sys::jint,
    JNIEnv
};

lazy_static!{
    static ref LIB: Library = Library::new("target/debug/libthird.dylib").unwrap();
    pub static ref div: Symbol<'static, extern fn(i32, i32) -> i32> = unsafe { LIB.get(b"div").unwrap()};
}

/// Returns the multiplication results of two integers
fn mult_internal(a: i32, b: i32) -> i32 {
    a * b
}

// **********************************
//              JNI API
// **********************************

#[no_mangle]
pub extern "system" fn Java_Main_mult(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    mult_internal(a, b)
}

#[no_mangle]
pub extern "system" fn Java_Main_sum(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    second::sum(a, b)
}

#[no_mangle]
pub extern "system" fn Java_Main_div(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    div(a, b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_mult() {
        assert_eq!(mult_internal(2, 3), 6);
        assert_eq!(mult_internal(10, 1), 10);
        assert_eq!(mult_internal(42, 0), 0);
    }
}
