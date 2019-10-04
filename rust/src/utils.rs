use std::mem;
use std::os::raw::c_int;

/// Does prep things to the given vec and returns the necessary information to
/// hurl it into the land of C.
///
/// May cause reallocation (the vec is `.shrink_to_fit()`ed).
pub fn vec_to_ffi<T>(mut vec: Vec<T>) -> (*mut T, c_int) {
    vec.shrink_to_fit();
    // Important property for later reconstruction on the Rust side
    assert!(vec.len() == vec.capacity());

    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    mem::forget(vec);

    (ptr, len as c_int)
}
