extern crate jni;

use jni::{
    objects::JClass,
    sys::jint,
    JNIEnv
};

/// Returns the result of substraction of two integers
fn sub_internal(a: i32, b:i32) -> i32 {
    a - b
}

/// Returns the sum of two integers
pub fn sum(a1: i32, a2: i32) -> i32 {
    a1 + a2
}

// **********************************
//              JNI API
// **********************************

#[no_mangle]
pub extern "system" fn Java_Main_sub(_env: JNIEnv, _class: JClass, a: jint, b:jint) -> jint {
    sub_internal(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_sum() {
        assert_eq!(sum(2, 3), 5);
    }

    #[test]
    fn second_sub() {
        assert_eq!(sub_internal(3, 2), 1);
        assert_eq!(sub_internal(10, 20), -10);
    }
}
