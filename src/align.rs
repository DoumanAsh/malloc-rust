use core::num;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
///Valid alignment representation
///
///## Requirements
///
///- Must be non-zero
///- Must be power of two
pub struct Alignment(num::NonZeroUsize);

impl Alignment {
    #[inline(always)]
    ///Creates new value, with panic if `alignment` doesn't fit requirements
    pub const fn new(alignment: usize) -> Self {
        assert!(alignment != 0, "Alignment cannot be zero");
        assert!(alignment.is_power_of_two());

        Self(unsafe {
            num::NonZeroUsize::new_unchecked(alignment)
        })
    }

    #[inline(always)]
    ///Creates new value, returning `None` if value doesn't fit requirements
    pub const fn try_new(alignment: usize) -> Option<Self> {
        match num::NonZeroUsize::new(alignment) {
            Some(value) if value.get().is_power_of_two() => Some(Self(value)),
            _ => None,
        }
    }

    #[inline(always)]
    ///Returns value at or after `size` that is a multiple of alignment.
    pub const fn next(&self, size: usize) -> usize {
        let alignment = self.0.get() - 1;
        size.saturating_add(alignment) & !alignment
    }

    #[inline(always)]
    ///Returns value at or before `size` that is a multiple of alignment.
    pub const fn prev(&self, size: usize) -> usize {
        let alignment = self.0.get() - 1;
        size & !alignment
    }

    #[inline(always)]
    ///Returns raw value
    pub const fn into_raw(&self) -> usize {
        self.0.get()
    }
}
