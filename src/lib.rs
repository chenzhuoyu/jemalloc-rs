use std::{
    error::Error,
    ffi::CString,
    fmt::{Display, Formatter, Result as FmtResult},
    mem::{size_of, zeroed},
    ptr::null_mut,
};

use sys::{
    extent_hooks_t, rog_free, rog_mallctl, rog_mallocx, MALLOCX_ALIGN, MALLOCX_ARENA,
    MALLOCX_TCACHE,
};

#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod bindings;

#[allow(non_snake_case)]
pub mod sys {
    pub use super::bindings::*;

    pub const MALLOCX_ZERO: libc::c_int = 0x40;
    pub const MALLOCX_TCACHE_NONE: libc::c_int = MALLOCX_TCACHE(-1i32 as u32);

    #[inline]
    pub const fn MALLOCX_LG_ALIGN(la: usize) -> libc::c_int {
        la as libc::c_int
    }

    #[inline]
    pub const fn MALLOCX_ALIGN(aling: usize) -> libc::c_int {
        aling.trailing_zeros() as libc::c_int
    }

    /// Bias tcache index bits so that 0 encodes "automatic tcache management",
    /// and 1 encodes MALLOCX_TCACHE_NONE.
    #[inline]
    pub const fn MALLOCX_TCACHE(tc: u32) -> libc::c_int {
        tc.wrapping_add(2).wrapping_shl(8) as libc::c_int
    }

    /// Bias arena index bits so that 0 encodes "use an automatically chosen arena".
    #[inline]
    pub const fn MALLOCX_ARENA(a: u32) -> libc::c_int {
        (a as libc::c_int).wrapping_add(1).wrapping_shl(20)
    }
}

#[derive(Debug, Clone)]
pub struct MallocCtrlError {
    name: String,
    errno: libc::c_int,
}

impl MallocCtrlError {
    pub fn new(err: libc::c_int, name: &str) -> Self {
        Self {
            name: name.to_string(),
            errno: err,
        }
    }
}

impl Error for MallocCtrlError {}

impl Display for MallocCtrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "mallctl({}) failed: [{}] {}",
            self.name,
            self.errno,
            unsafe { CString::from_raw(libc::strerror(self.errno)) }.to_string_lossy()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Arena {
    id: u32,
}

impl Arena {
    pub fn with_hooks(hooks: &'static extent_hooks_t) -> Result<Self, MallocCtrlError> {
        Ok(Self {
            id: unsafe {
                invoke_mallctl(
                    "arenas.create",
                    hooks as *const _ as *mut extent_hooks_t as *mut _,
                    size_of::<extent_hooks_t>(),
                )?
            },
        })
    }
}

impl Arena {
    pub fn free(self, ptr: &mut libc::c_void) {
        unsafe {
            rog_free(ptr);
        }
    }

    pub fn malloc_with_tcache(
        self,
        size: usize,
        align: usize,
        tcache: TCache,
    ) -> *mut libc::c_void {
        unsafe {
            rog_mallocx(
                size,
                MALLOCX_ARENA(self.id) | MALLOCX_TCACHE(tcache.id) | MALLOCX_ALIGN(align),
            )
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TCache {
    id: u32,
}

impl TCache {
    pub fn new() -> Result<Self, MallocCtrlError> {
        Ok(Self {
            id: unsafe { invoke_mallctl("tcache.create", null_mut(), 0)? },
        })
    }
}

unsafe fn invoke_mallctl<R: Sized>(
    name: &str,
    newp: *mut libc::c_void,
    newlen: usize,
) -> Result<R, MallocCtrlError> {
    let mut old = zeroed::<R>();
    let mut len = size_of::<R>();

    /* invoke `mallctl` with wrapped key name */
    let key = CString::new(name).map_err(|_| MallocCtrlError::new(libc::EINVAL, name))?;
    let err = rog_mallctl(
        key.as_ptr(),
        &mut old as *mut _ as *mut libc::c_void,
        &mut len as *mut _,
        newp,
        newlen,
    );

    /* check for errors */
    if err == 0 {
        Ok(old)
    } else {
        Err(MallocCtrlError::new(err, name))
    }
}

#[cfg(test)]
mod tests {
    use crate::sys;

    #[test]
    fn should_allocate_memory() {
        let ptr = unsafe { sys::rog_malloc(16) };
        dbg!(ptr);
        assert!(!ptr.is_null(), "jemalloc is unable to allocate memory");
        unsafe { sys::rog_free(ptr) };
    }
}
