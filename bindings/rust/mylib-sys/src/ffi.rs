/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sum {
    pub value: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Sum"][::std::mem::size_of::<Sum>() - 4usize];
    ["Alignment of Sum"][::std::mem::align_of::<Sum>() - 4usize];
    ["Offset of field: Sum::value"][::std::mem::offset_of!(Sum, value) - 0usize];
};
unsafe extern "C" {
    pub fn print_hello() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn add_int(
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
        sum: *mut Sum,
    ) -> ::std::os::raw::c_int;
}
