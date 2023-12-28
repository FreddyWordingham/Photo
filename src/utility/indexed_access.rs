//! Indexed access trait.

/// Types implementing this trait can be accessed by index.
pub trait IndexedAccess<T> {
    fn retrieve(&self, index: usize) -> T;
}
