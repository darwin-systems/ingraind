/* automatically generated by rust-bindgen */

pub type __u64 = ::std::os::raw::c_ulonglong;
pub type perf_reader_raw_cb = ::std::option::Option<
    unsafe extern "C" fn(
        cb_cookie: *mut ::std::os::raw::c_void,
        raw: *mut ::std::os::raw::c_void,
        raw_size: ::std::os::raw::c_int,
    ),
>;
pub type perf_reader_lost_cb =
    ::std::option::Option<unsafe extern "C" fn(cb_cookie: *mut ::std::os::raw::c_void, lost: u64)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct perf_reader {
    _unused: [u8; 0],
}
extern "C" {
    pub fn perf_reader_new(
        raw_cb: perf_reader_raw_cb,
        lost_cb: perf_reader_lost_cb,
        cb_cookie: *mut ::std::os::raw::c_void,
        page_cnt: ::std::os::raw::c_int,
    ) -> *mut perf_reader;
}
extern "C" {
    pub fn perf_reader_free(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn perf_reader_mmap(reader: *mut perf_reader) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perf_reader_event_read(reader: *mut perf_reader);
}
extern "C" {
    pub fn perf_reader_poll(
        num_readers: ::std::os::raw::c_int,
        readers: *mut *mut perf_reader,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perf_reader_fd(reader: *mut perf_reader) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perf_reader_set_fd(reader: *mut perf_reader, fd: ::std::os::raw::c_int);
}