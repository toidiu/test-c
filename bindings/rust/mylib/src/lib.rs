use mylib_sys::ffi;

pub fn add(left: i32, right: i32) -> i32 {
    let mut sum = ffi::Sum { value: 0 };
    unsafe { ffi::add_int(left, right, (&mut sum) as *mut ffi::Sum) };
    sum.value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
