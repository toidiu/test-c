use std::os::raw::c_int;

// #[repr(C)]
// pub struct Sum {
//     pub value: c_int,
// }

extern "C" {
    pub fn print_hello() -> c_int;

    //     pub fn add_int(a: c_int, b: c_int, sum: *mut Sum) -> c_int;
    //     // pub fn add(i: ::std::os::raw::c_int, c: ::std::os::raw::c_char, cs: *mut CoolStruct);
}

// extern "C" {
// }

// int print_hello()
// int add_int(int a, int b, struct Sum *sum)
