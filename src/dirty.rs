use core::mem;
use core::ops::{ Deref, DerefMut };


pub struct Dirty<T> {
    value : T,
    dirty : bool
}

impl<T> Dirty<T> {

    /// Creates a new `Dirty` not marked as dirty.
    pub const fn new_clean(value : T) -> Self {
        Self { value, dirty : false }
    }

    /// Creates a new `Dirty` already marked as dirty.
    pub const fn new_dirty(value : T) -> Self {
        Self { value, dirty : true }
    }

}

impl<T : PartialEq> Dirty<T> {

    /// Sets the value contained in the `Dirty` to a value,
    ///  marking it as dirty if the old value does not equal
    ///  the new value.
    ///
    /// For a method which does not check equality and
    ///  unconditionally marks the `Dirty` as dirty, see
    ///  [`Dirty::set_dirty`].
    pub fn set(container : &mut Self, value : T) {
        if (container.value != value) {
            Self::mark_dirty(container);
        }
        Self::set_silent(container, value);
    }

}

impl<T> Dirty<T> {

    /// Sets the value contained in the `Dirty` to a value,
    ///  marking it as dirty.
    ///
    /// This does not check if the old and new values are
    ///  equivalent. For a method which checks equality,
    ///  see [`Dirty::set`].
    pub fn set_dirty(container : &mut Self, value : T) {
        Self::set_silent(container, value);
        Self::mark_dirty(container);
    }

    /// Sets the value contained in the Dirty to a value,
    ///  leaving its dirty state unchanged.
    ///
    /// If the Dirty was already marked dirty,
    ///  it will stay marked dirty.
    #[inline]
    pub fn set_silent(container : &mut Self, value : T) {
        container.value = value;
    }

}

impl<T> Dirty<T> {

    /// Returns an immutable reference to the contained
    ///  value.
    #[inline]
    pub fn get_ref(container : &mut Self) -> &T {
        &container.value
    }

}

impl<T : Clone + Copy + PartialEq> Dirty<T> {

    /// Returns a mutable reference to the contained
    ///  value.
    #[inline(always)]
    pub fn get_mut<'l>(container : &'l mut Self) -> DirtyMut<'l, T> {
        DirtyMut { container, original_value : None }
    }

}

impl<T> Dirty<T> {

    /// Marks the `Dirty` as dirty without changing the
    ///  contained value.
    #[inline]
    pub fn mark_dirty(container : &mut Self) {
        container.dirty = true;
    }

    // Unmarks the `Dirty` as dirty without changing the
    //  contained value.
    #[inline]
    pub fn mark_clean(container : &mut Self) {
        container.dirty = false;
    }

    // Sets whether the `Dirty` is marked as dirty
    //  without changing the contained value.
    #[inline]
    pub fn mark(container : &mut Self, dirty : bool) {
        container.dirty = dirty;
    }

}

impl<T> Dirty<T> {

    /// Returns `true` if the `Dirty` is marked as dirty.
    #[inline]
    pub fn is_dirty(container : &mut Self) -> bool {
        container.dirty
    }

    /// Returns `true` if the `Dirty` is marked as dirty
    ///  and unmarks it as dirty.
    pub fn take_dirty(container : &mut Self) -> bool {
        mem::replace(&mut container.dirty, false)
    }

}

impl<T> Deref for Dirty<T> {

    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }

}


pub struct DirtyMut<'l, T : Clone + PartialEq> {
    container      : &'l mut Dirty<T>,
    original_value : Option<T>
}

impl<'l, T : Clone + PartialEq> Deref for DirtyMut<'l, T> {

    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.container.value
    }

}

impl<'l, T : Clone + PartialEq> DerefMut for DirtyMut<'l, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        if (self.original_value.is_none()) {
            self.original_value = Some(self.container.value.clone());
        }
        &mut self.container.value
    }

}

impl<'l, T : Clone + PartialEq> Drop for DirtyMut<'l, T> {

    fn drop(&mut self) {
        if let Some(original_value) = &self.original_value {
            if (original_value != &self.container.value) {
                Dirty::mark_dirty(self.container);
            }
        }
    }

}
