extern crate jni;

use jni::{
    objects::JClass,
    sys::jint,
    JNIEnv
};

/// Returns the square of integer.
fn sqr_internal(a: i32) -> i32 {
    a * a
}

/// Returns result of division of two integers. Exported function, `no_mangle` is required.
#[no_mangle]
pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

// **********************************
//              JNI API
// **********************************
#[no_mangle]
pub extern "system" fn Java_Main_sqr(_env: JNIEnv, _class: JClass, a: jint) -> jint {
    sqr_internal(a)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn third_div() {
        assert_eq!(div(42, 6), 7);
    }

    #[test]
    fn third_sqr() {
        assert_eq!(sqr_internal(0), 0);
        assert_eq!(sqr_internal(1), 1);
        assert_eq!(sqr_internal(2), 4);
        assert_eq!(sqr_internal(4), 16);
    }
}
