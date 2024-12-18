mod ffi;

pub fn add(left: i32, right: i32) -> i32 {
    // let sum = ffi::Sum { value: 0 };
    // unsafe { ffi::add_int(left, right, sum as *mut ffi::Sum) };
    unsafe { ffi::print_hello() };
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe { ffi::print_hello() };
    }
}
