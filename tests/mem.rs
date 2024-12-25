use malloc_rust::DEFAULT_ALIGNMENT;
use malloc_rust::{rust_calloc, rust_malloc, rust_size, rust_free, generic_try_rust_malloc, rust_realloc, generic_try_rust_realloc};
use core::{mem, slice};

#[test]
fn mem_should_verify_malloc_alignment() {
    for size in 1..=99 {
        let expected_size = DEFAULT_ALIGNMENT.next(size).saturating_add(mem::size_of::<usize>());
        unsafe {
            let ptr = rust_malloc(size);
            assert!(!ptr.is_null());
            assert!(ptr.is_aligned());
            assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());
            rust_free(ptr);
        }
    }
}

#[test]
fn mem_should_malloc_memory() {
    const SIZE: usize = 10;
    const INT_SIZE: i32 = SIZE as _;

    let expected_size = DEFAULT_ALIGNMENT.next(SIZE).saturating_add(mem::size_of::<usize>());
    unsafe {
        let ptr = rust_malloc(SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());

        let ptr = rust_realloc(ptr, SIZE + 1);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());
        rust_free(ptr);

        let ptr = generic_try_rust_malloc(INT_SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());
        let ptr = generic_try_rust_realloc(ptr, INT_SIZE + 1);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());
        rust_free(ptr);
    }
}

#[test]
fn mem_should_calloc_memory() {
    const SIZE: usize = 10;

    let expected_size = DEFAULT_ALIGNMENT.next(SIZE).saturating_add(mem::size_of::<usize>());
    unsafe {
        let ptr = rust_calloc(SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());
        let slice = slice::from_raw_parts(ptr as *const u8, expected_size - mem::size_of::<usize>());
        assert!(slice.iter().all(|byt| *byt == 0));

        let ptr = rust_realloc(ptr, SIZE + 1);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size - mem::size_of::<usize>());
        rust_free(ptr);
    }
}

#[test]
fn mem_should_handle_zero_allocation() {
    const SIZE: usize = 0;
    const INT_SIZE: i32 = SIZE as _;

    unsafe {
        let ptr = rust_malloc(SIZE);
        assert!(ptr.is_null());
        rust_free(ptr);

        let ptr = generic_try_rust_malloc(INT_SIZE);
        assert!(ptr.is_null());
        rust_free(ptr);
    }
}

#[test]
fn mem_should_handle_isize_overflow() {
    const SIZE: usize = isize::MAX as usize + 1;
    const INT_SIZE: i64 = SIZE as _;

    unsafe {
        let ptr = rust_malloc(SIZE);
        assert!(ptr.is_null());
        rust_free(ptr);

        let ptr = generic_try_rust_malloc(INT_SIZE);
        assert!(ptr.is_null());
        rust_free(ptr);
    }
}
