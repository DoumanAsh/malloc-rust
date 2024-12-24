use malloc_rust::{DEFAULT_ALIGNMENT, rust_malloc, rust_size, rust_free, generic_try_rust_malloc, rust_realloc, generic_try_rust_realloc};
use core::mem;

#[test]
fn mem_should_allocate_memory() {
    const SIZE: usize = 10;
    const INT_SIZE: i32 = SIZE as _;

    let expected_size = DEFAULT_ALIGNMENT.next(SIZE + mem::size_of::<usize>());
    unsafe {
        let ptr = rust_malloc(SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size);

        let ptr = rust_realloc(ptr, SIZE + 1);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size);
        rust_free(ptr);

        let ptr = generic_try_rust_malloc(INT_SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size);
        let ptr = generic_try_rust_realloc(ptr, INT_SIZE + 1);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        assert_eq!(rust_size(ptr), expected_size);
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
