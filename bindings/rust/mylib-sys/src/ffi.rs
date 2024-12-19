use std::os::raw::c_int;

#[repr(C)]
pub struct Sum {
    pub value: c_int,
}

extern "C" {
    pub fn print_hello() -> c_int;

    pub fn add_int(a: c_int, b: c_int, sum: *mut Sum) -> c_int;
}
