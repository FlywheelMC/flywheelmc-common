pub trait Increment {
    /// Increments `self`, returning the old value.
    fn increment(&mut self) -> Self;
}

macro_rules! impl_inc {(
    $ty:ty
) => {
    impl Increment for $ty {
        fn increment(&mut self) -> Self {
            let out = *self;
            *self = self.wrapping_add(1);
            out
        }
    }
}}

impl_inc!(u8);
impl_inc!(i8);
impl_inc!(u16);
impl_inc!(i16);
impl_inc!(u32);
impl_inc!(i32);
impl_inc!(u64);
impl_inc!(i64);
impl_inc!(u128);
impl_inc!(i128);
impl_inc!(usize);
impl_inc!(isize);
