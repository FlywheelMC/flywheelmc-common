use core::mem;
use core::ops::Deref;


pub struct Ordered<T> {
    value   : T,
    min_gen : u128,
    dirty   : bool
}

impl<T> Ordered<T> {

    pub const fn new(value : T) -> Self {
        Self { value, min_gen : 0, dirty : false }
    }

}

impl<T : PartialEq> Ordered<T> {

    pub fn set(container : &mut Self, value : T, generation : u128) {
        if (generation >= container.min_gen) {
            if (container.value != value) {
                Self::mark_dirty(container);
            }
            Self::set_silent(container, value);
            container.min_gen = generation + 1;
        }
    }

    pub fn set_nogen(container : &mut Self, value : T) {
        if (container.value != value) {
            Self::mark_dirty(container);
        }
        Self::set_silent(container, value);
    }

}

impl<T> Ordered<T> {

    pub fn set_dirty(container : &mut Self, value : T) {
        Self::set_silent(container, value);
        Self::mark_dirty(container);
    }

    #[inline]
    pub fn set_silent(container : &mut Self, value : T) {
        container.value = value;
    }

}

impl<T> Ordered<T> {

    /// Returns an immutable reference to the contained
    ///  value.
    #[inline]
    pub fn get_ref(container : &mut Self) -> &T {
        &container.value
    }

}

impl<T> Ordered<T> {

    #[inline]
    pub fn mark_dirty(container : &mut Self) {
        container.dirty = true;
    }

    #[inline]
    pub fn mark_clean(container : &mut Self) {
        container.dirty = false;
    }

    #[inline]
    pub fn mark(container : &mut Self, dirty : bool) {
        container.dirty = dirty;
    }

}

impl<T> Ordered<T> {

    #[inline]
    pub fn is_dirty(container : &mut Self) -> bool {
        container.dirty
    }
    pub fn take_dirty(container : &mut Self) -> bool {
        mem::replace(&mut container.dirty, false)
    }

}

impl<T> Deref for Ordered<T> {

    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }

}
