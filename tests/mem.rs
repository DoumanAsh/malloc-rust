use malloc_rust::{rust_malloc, rust_free, generic_try_rust_malloc};

#[test]
fn mem_should_allocate_memory() {
    const SIZE: usize = 10;
    const INT_SIZE: i32 = SIZE as _;

    unsafe {
        let ptr = rust_malloc(SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        rust_free(ptr);

        let ptr = generic_try_rust_malloc(INT_SIZE);
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
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
