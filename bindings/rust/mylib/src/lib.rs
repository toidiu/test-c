use mylib_sys::ffi;
use mylib_sys::manual_ffi;

pub fn add(left: i32, right: i32) -> i32 {
    let mut sum = ffi::Sum { value: 0 };
    unsafe { ffi::add_int(left, right, (&mut sum) as *mut ffi::Sum) };
    sum.value
}

pub fn manual_add(left: i32, right: i32) -> i32 {
    let mut sum = manual_ffi::Sum { value: 0 };
    unsafe { manual_ffi::add_int(left, right, (&mut sum) as *mut manual_ffi::Sum) };
    sum.value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let ffi_r = add(2, 2);
        assert_eq!(ffi_r, 4);

        let manual_ffi_r = add(2, 2);
        assert_eq!(manual_ffi_r, 4);

        assert_eq!(manual_ffi_r, ffi_r);
    }
}
