//! Malloc implementation using Rust allocator
//!
//! This crate MUST not have semver breaking changes

#![no_std]
#![allow(clippy::style)]

extern crate alloc;

pub mod align;

use core::{mem, ptr};
use core::ffi::c_void;
use core::convert::TryInto;
use alloc::alloc::Layout;

///Default alignment.
///
///On most platforms default value is 8 with exception of Mac OS and windows 64bit which uses 16
pub const DEFAULT_ALIGNMENT: align::Alignment = {
    #[cfg(any(target_os = "macos", all(windows, target_pointer_width = "64")))]
    {
        align::Alignment::new(16)
    }
    #[cfg(not(any(target_os = "macos", all(windows, target_pointer_width = "64"))))]
    {
        align::Alignment::new(8)
    }
};

const LAYOUT_OFFSET: usize = mem::size_of::<usize>();

#[cold]
#[inline(never)]
fn unlikely_null() -> *mut c_void {
    ptr::null_mut()
}

#[inline]
///Baseline `malloc` implementation with Rust allocator
///
///Returns NULL if size is 0 or overflows `isize::MAX`
pub unsafe extern "C" fn rust_malloc(mut size: usize) -> *mut c_void {
    if size != 0 {
        size = LAYOUT_OFFSET.saturating_add(DEFAULT_ALIGNMENT.next(size));

        if let Ok(layout) = Layout::from_size_align(size, DEFAULT_ALIGNMENT.into_raw()) {
            let mem = alloc::alloc::alloc(layout);
            if !mem.is_null() {
                ptr::write(mem as *mut usize, size);
                return mem.add(LAYOUT_OFFSET) as _;
            }
        }
    }

    unlikely_null()
}

#[inline]
///Generic `malloc` implementation which requires size to be converted into `usize` without error
pub unsafe extern "C" fn generic_rust_malloc<T: Into<usize>>(size: T) -> *mut c_void {
    rust_malloc(size.into())
}

#[inline]
///Generic `malloc` implementation which allows size to be optionally convertable.
///
///In case of invalid size, returns NULL pointer
pub unsafe extern "C" fn generic_try_rust_malloc<T: TryInto<usize>>(size: T) -> *mut c_void {
    if let Ok(size) = size.try_into() {
        rust_malloc(size)
    } else {
        unlikely_null()
    }
}

#[inline]
///Baseline `realloc` implementation with Rust allocator
///
///Returns NULL if size is 0 or overflows `isize::MAX`
pub unsafe extern "C" fn rust_realloc(mut old_ptr: *mut c_void, mut new_size: usize) -> *mut c_void {
    if new_size != 0 {
        new_size = LAYOUT_OFFSET.saturating_add(DEFAULT_ALIGNMENT.next(new_size));

        old_ptr = (old_ptr as *mut u8).offset(-(LAYOUT_OFFSET as isize)) as _;
        let size = ptr::read(old_ptr as *const usize);
        let layout = Layout::from_size_align_unchecked(size, DEFAULT_ALIGNMENT.into_raw());
        let new_ptr = alloc::alloc::realloc(old_ptr as _, layout, new_size);
        if !new_ptr.is_null() {
            ptr::write(new_ptr as *mut usize, new_size);
            return new_ptr.add(LAYOUT_OFFSET) as _;
        }
    }

    unlikely_null()
}

#[inline]
///Generic `realloc` implementation which requires size to be converted into `usize` without error
pub unsafe extern "C" fn generic_rust_realloc<T: Into<usize>>(mem: *mut c_void, size: T) -> *mut c_void {
    rust_realloc(mem, size.into())
}

#[inline]
///Generic `realloc` implementation which allows size to be optionally convertable.
///
///In case of invalid size, returns NULL pointer
pub unsafe extern "C" fn generic_try_rust_realloc<T: TryInto<usize>>(mem: *mut c_void, size: T) -> *mut c_void {
    if let Ok(size) = size.try_into() {
        rust_realloc(mem, size)
    } else {
        unlikely_null()
    }
}

#[inline]
///Baseline `calloc` implementation with Rust allocator
///
///Returns NULL if size is 0 or overflows `isize::MAX`
pub unsafe extern "C" fn rust_calloc(mut size: usize) -> *mut c_void {
    if size != 0 {
        size = LAYOUT_OFFSET.saturating_add(DEFAULT_ALIGNMENT.next(size));

        if let Ok(layout) = Layout::from_size_align(size, DEFAULT_ALIGNMENT.into_raw()) {
            let mem = alloc::alloc::alloc_zeroed(layout);
            if !mem.is_null() {
                ptr::write(mem as *mut usize, size);
                return mem.add(LAYOUT_OFFSET) as _;
            }
        }
    }

    unlikely_null()
}

#[inline]
///Returns size of allocated memory in pointer
///
///Returns 0 for NULL
pub unsafe extern "C" fn rust_size(mem: *mut c_void) -> usize {
    if !mem.is_null() {
        let mem = (mem as *mut u8).offset(-(LAYOUT_OFFSET as isize));
        let size = ptr::read(mem as *const usize);
        size.saturating_sub(LAYOUT_OFFSET)
    } else {
        0
    }
}

#[inline]
///Baseline `free` implementation with Rust allocator
///
///Does nothing if `mem` is null
pub unsafe extern "C" fn rust_free(mem: *mut c_void) {
    if !mem.is_null() {
        let mem = (mem as *mut u8).offset(-(LAYOUT_OFFSET as isize));
        let size = ptr::read(mem as *const usize);
        let layout = Layout::from_size_align_unchecked(size, DEFAULT_ALIGNMENT.into_raw());
        alloc::alloc::dealloc(mem, layout);
    }
}
