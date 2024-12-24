use malloc_rust::align::Alignment;

#[test]
fn align_should_try_create_valid_only() {
    for alignment in 0..=512 {
        let result = Alignment::try_new(alignment);
        if alignment.is_power_of_two() {
            assert!(result.is_some(), "Alignment is power of two, but result is not valid");
        } else {
            assert!(result.is_none(), "Alignment is not power of two, but result is valid");
        }
    }
}

#[should_panic]
#[test]
fn align_should_should_panic_on_zero() {
    Alignment::new(0);
}

#[should_panic]
#[test]
fn align_should_should_panic_on_not_power_of_two() {
    Alignment::new(3);
}

#[test]
fn align_should_verify_next_round() {
    let alignment = Alignment::new(4);
    assert_eq!(alignment.next(9), 12);
    assert_eq!(alignment.next(13), 16);
    assert_eq!(alignment.next(12), 12);
    assert_eq!(alignment.next(4), 4);
    assert_eq!(alignment.next(8), 8);
    assert_eq!(alignment.next(0), 0);

    let alignment = Alignment::new(1);
    assert_eq!(alignment.next(13), 13);
    assert_eq!(alignment.next(0), 0);

    let alignment = Alignment::new(512);
    assert_eq!(alignment.next(16), 512);
}

#[test]
fn align_should_verify_prev_round() {
    let alignment = Alignment::new(4);
    assert_eq!(alignment.prev(9), 8);
    assert_eq!(alignment.prev(13), 12);
    assert_eq!(alignment.prev(12), 12);
    assert_eq!(alignment.prev(4), 4);
    assert_eq!(alignment.prev(8), 8);
    assert_eq!(alignment.prev(0), 0);

    let alignment = Alignment::new(1);
    assert_eq!(alignment.prev(13), 13);
    assert_eq!(alignment.prev(0), 0);

    let alignment = Alignment::new(512);
    assert_eq!(alignment.prev(16), 0);
}
