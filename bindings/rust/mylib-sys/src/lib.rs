pub mod ffi;

#[cfg(test)]
mod tests {
    use super::*;

    pub fn add(left: i32, right: i32) -> i32 {
        let mut sum = ffi::Sum { value: 0 };
        unsafe { ffi::add_int(left, right, (&mut sum) as *mut ffi::Sum) };
        sum.value
    }

    #[test]
    fn linking_works() {
        unsafe { ffi::print_hello() };
    }

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 3), 3);
        assert_eq!(add(0, 0), 0);
    }
}
