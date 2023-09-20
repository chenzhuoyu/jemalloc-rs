/* automatically generated by rust-bindgen 0.68.1 */

pub const JEMALLOC_VERSION: &[u8; 50] = b"0.0.0-0-g000000missing_version_try_git_fetch_tags\0";
pub const JEMALLOC_VERSION_MAJOR: u32 = 0;
pub const JEMALLOC_VERSION_MINOR: u32 = 0;
pub const JEMALLOC_VERSION_BUGFIX: u32 = 0;
pub const JEMALLOC_VERSION_NREV: u32 = 0;
pub const JEMALLOC_VERSION_GID: &[u8; 13] = b"000000missin\0";
pub const MALLCTL_ARENAS_ALL: u32 = 4096;
pub const MALLCTL_ARENAS_DESTROYED: u32 = 4097;
extern "C" {
    pub fn rog_malloc(size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn rog_calloc(num: usize, size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn rog_posix_memalign(
        memptr: *mut *mut libc::c_void,
        alignment: usize,
        size: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn rog_aligned_alloc(alignment: usize, size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn rog_realloc(ptr: *mut libc::c_void, size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn rog_free(ptr: *mut libc::c_void);
}
extern "C" {
    pub fn rog_mallocx(size: usize, flags: libc::c_int) -> *mut libc::c_void;
}
extern "C" {
    pub fn rog_rallocx(
        ptr: *mut libc::c_void,
        size: usize,
        flags: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn rog_xallocx(
        ptr: *mut libc::c_void,
        size: usize,
        extra: usize,
        flags: libc::c_int,
    ) -> usize;
}
extern "C" {
    pub fn rog_sallocx(ptr: *const libc::c_void, flags: libc::c_int) -> usize;
}
extern "C" {
    pub fn rog_dallocx(ptr: *mut libc::c_void, flags: libc::c_int);
}
extern "C" {
    pub fn rog_sdallocx(ptr: *mut libc::c_void, size: usize, flags: libc::c_int);
}
extern "C" {
    pub fn rog_nallocx(size: usize, flags: libc::c_int) -> usize;
}
extern "C" {
    pub fn rog_mallctl(
        name: *const libc::c_char,
        oldp: *mut libc::c_void,
        oldlenp: *mut usize,
        newp: *mut libc::c_void,
        newlen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn rog_mallctlnametomib(
        name: *const libc::c_char,
        mibp: *mut usize,
        miblenp: *mut usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn rog_mallctlbymib(
        mib: *const usize,
        miblen: usize,
        oldp: *mut libc::c_void,
        oldlenp: *mut usize,
        newp: *mut libc::c_void,
        newlen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn rog_malloc_stats_print(
        write_cb: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut libc::c_void, arg2: *const libc::c_char),
        >,
        je_cbopaque: *mut libc::c_void,
        opts: *const libc::c_char,
    );
}
extern "C" {
    pub fn rog_malloc_usable_size(ptr: *const libc::c_void) -> usize;
}
extern "C" {
    pub fn rog_valloc(size: usize) -> *mut libc::c_void;
}
pub type extent_hooks_t = extent_hooks_s;
pub type extent_alloc_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: usize,
        arg5: *mut bool,
        arg6: *mut bool,
        arg7: libc::c_uint,
    ) -> *mut libc::c_void,
>;
pub type extent_dalloc_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: bool,
        arg5: libc::c_uint,
    ) -> bool,
>;
pub type extent_destroy_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: bool,
        arg5: libc::c_uint,
    ),
>;
pub type extent_commit_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: libc::c_uint,
    ) -> bool,
>;
pub type extent_decommit_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: libc::c_uint,
    ) -> bool,
>;
pub type extent_purge_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: libc::c_uint,
    ) -> bool,
>;
pub type extent_split_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: bool,
        arg7: libc::c_uint,
    ) -> bool,
>;
pub type extent_merge_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut extent_hooks_t,
        arg2: *mut libc::c_void,
        arg3: usize,
        arg4: *mut libc::c_void,
        arg5: usize,
        arg6: bool,
        arg7: libc::c_uint,
    ) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct extent_hooks_s {
    pub alloc: extent_alloc_t,
    pub dalloc: extent_dalloc_t,
    pub destroy: extent_destroy_t,
    pub commit: extent_commit_t,
    pub decommit: extent_decommit_t,
    pub purge_lazy: extent_purge_t,
    pub purge_forced: extent_purge_t,
    pub split: extent_split_t,
    pub merge: extent_merge_t,
}
#[test]
fn bindgen_test_layout_extent_hooks_s() {
    const UNINIT: ::std::mem::MaybeUninit<extent_hooks_s> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<extent_hooks_s>(),
        72usize,
        concat!("Size of: ", stringify!(extent_hooks_s))
    );
    assert_eq!(
        ::std::mem::align_of::<extent_hooks_s>(),
        8usize,
        concat!("Alignment of ", stringify!(extent_hooks_s))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alloc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(alloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dalloc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(dalloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).destroy) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(destroy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).commit) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(commit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).decommit) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(decommit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).purge_lazy) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(purge_lazy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).purge_forced) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(purge_forced)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).split) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(split)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).merge) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(extent_hooks_s),
            "::",
            stringify!(merge)
        )
    );
}
