use core::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Rem, RemAssign,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Shl, ShlAssign,
    Shr, ShrAssign,
    Neg,
    Not
};


auto trait IsNotVector { }

macro_rules! strip_plus {(
    + $( $tt:tt )*
) => { $( $tt )* }}


macro_rules! vec {(
    $vis:vis $ident:ident { $( $field:ident ),* $(,)? }
) => {

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    $vis struct $ident<T> {
        $( $vis $field : T , )*
    }

    impl<T> !IsNotVector for $ident<T> { }

    impl<T> $ident<T> {

        /// Creates a new vector with the given elements.
        #[inline]
        $vis const fn new( $( $field : T , )* ) -> Self {
            Self { $( $field , )* }
        }

        /// Creates a new vector with the given value in all elements.
        #[inline]
        $vis const fn splat(v : T) -> Self
        where
            T : Copy
        { Self { $( $field : v , )* } }

        /// Create a new vector from an array.
        #[inline]
        $vis const fn from_array([ $( $field , )* ] : [T; ${count($field)}]) -> Self
        where
            T : Copy
        { Self { $( $field , )* } }

    }

    impl $ident<bool> {
        $vis const FALSE : Self = Self::splat(false);
        $vis const TRUE  : Self = Self::splat(true);
    }
    impl $ident<u8> {
        $vis const ZERO : Self = Self::splat(0);
        $vis const ONE  : Self = Self::splat(1);
        $vis const MIN  : Self = Self::splat(u8::MIN);
        $vis const MAX  : Self = Self::splat(u8::MAX);
    }
    impl $ident<i8> {
        $vis const ZERO    : Self = Self::splat(0);
        $vis const ONE     : Self = Self::splat(1);
        $vis const NEG_ONE : Self = Self::splat(-1);
        $vis const MIN     : Self = Self::splat(i8::MIN);
        $vis const MAX     : Self = Self::splat(i8::MAX);
    }
    impl $ident<u16> {
        $vis const ZERO : Self = Self::splat(0);
        $vis const ONE  : Self = Self::splat(1);
        $vis const MIN  : Self = Self::splat(u16::MIN);
        $vis const MAX  : Self = Self::splat(u16::MAX);
    }
    impl $ident<i16> {
        $vis const ZERO    : Self = Self::splat(0);
        $vis const ONE     : Self = Self::splat(1);
        $vis const NEG_ONE : Self = Self::splat(-1);
        $vis const MIN     : Self = Self::splat(i16::MIN);
        $vis const MAX     : Self = Self::splat(i16::MAX);
    }
    impl $ident<u32> {
        $vis const ZERO : Self = Self::splat(0);
        $vis const ONE  : Self = Self::splat(1);
        $vis const MIN  : Self = Self::splat(u32::MIN);
        $vis const MAX  : Self = Self::splat(u32::MAX);
    }
    impl $ident<i32> {
        $vis const ZERO    : Self = Self::splat(0);
        $vis const ONE     : Self = Self::splat(1);
        $vis const NEG_ONE : Self = Self::splat(-1);
        $vis const MIN     : Self = Self::splat(i32::MIN);
        $vis const MAX     : Self = Self::splat(i32::MAX);
    }
    impl $ident<u64> {
        $vis const ZERO : Self = Self::splat(0);
        $vis const ONE  : Self = Self::splat(1);
        $vis const MIN  : Self = Self::splat(u64::MIN);
        $vis const MAX  : Self = Self::splat(u64::MAX);
    }
    impl $ident<i64> {
        $vis const ZERO    : Self = Self::splat(0);
        $vis const ONE     : Self = Self::splat(1);
        $vis const NEG_ONE : Self = Self::splat(-1);
        $vis const MIN     : Self = Self::splat(i64::MIN);
        $vis const MAX     : Self = Self::splat(i64::MAX);
    }
    impl $ident<u128> {
        $vis const ZERO : Self = Self::splat(0);
        $vis const ONE  : Self = Self::splat(1);
        $vis const MIN  : Self = Self::splat(u128::MIN);
        $vis const MAX  : Self = Self::splat(u128::MAX);
    }
    impl $ident<i128> {
        $vis const ZERO    : Self = Self::splat(0);
        $vis const ONE     : Self = Self::splat(1);
        $vis const NEG_ONE : Self = Self::splat(-1);
        $vis const MIN     : Self = Self::splat(i128::MIN);
        $vis const MAX     : Self = Self::splat(i128::MAX);
    }
    impl $ident<usize> {
        $vis const ZERO : Self = Self::splat(0);
        $vis const ONE  : Self = Self::splat(1);
        $vis const MIN  : Self = Self::splat(usize::MIN);
        $vis const MAX  : Self = Self::splat(usize::MAX);
    }
    impl $ident<isize> {
        $vis const ZERO    : Self = Self::splat(0);
        $vis const ONE     : Self = Self::splat(1);
        $vis const NEG_ONE : Self = Self::splat(-1);
        $vis const MIN     : Self = Self::splat(isize::MIN);
        $vis const MAX     : Self = Self::splat(isize::MAX);
    }
    impl $ident<f32> {
        $vis const ZERO         : Self = Self::splat(0.0);
        $vis const ONE          : Self = Self::splat(1.0);
        $vis const NEG_ONE      : Self = Self::splat(-1.0);
        $vis const MIN          : Self = Self::splat(f32::MIN);
        $vis const MAX          : Self = Self::splat(f32::MAX);
        $vis const NAN          : Self = Self::splat(f32::NAN);
        $vis const INFINITY     : Self = Self::splat(f32::INFINITY);
        $vis const NEG_INFINITY : Self = Self::splat(f32::NEG_INFINITY);
    }
    impl $ident<f64> {
        $vis const ZERO         : Self = Self::splat(0.0);
        $vis const ONE          : Self = Self::splat(1.0);
        $vis const NEG_ONE      : Self = Self::splat(-1.0);
        $vis const MIN          : Self = Self::splat(f64::MIN);
        $vis const MAX          : Self = Self::splat(f64::MAX);
        $vis const NAN          : Self = Self::splat(f64::NAN);
        $vis const INFINITY     : Self = Self::splat(f64::INFINITY);
        $vis const NEG_INFINITY : Self = Self::splat(f64::NEG_INFINITY);
    }

    vec_op_binary! { $ident { $( $field , )* } impl Add          add           +   }
    vec_op_assign! { $ident { $( $field , )* } impl AddAssign    add_assign    +=  }
    vec_op_binary! { $ident { $( $field , )* } impl Sub          sub           -   }
    vec_op_assign! { $ident { $( $field , )* } impl SubAssign    sub_assign    -=  }
    vec_op_binary! { $ident { $( $field , )* } impl Mul          mul           *   }
    vec_op_assign! { $ident { $( $field , )* } impl MulAssign    mul_assign    *=  }
    vec_op_binary! { $ident { $( $field , )* } impl Div          div           /   }
    vec_op_assign! { $ident { $( $field , )* } impl DivAssign    div_assign    /=  }
    vec_op_binary! { $ident { $( $field , )* } impl Rem          rem           %   }
    vec_op_assign! { $ident { $( $field , )* } impl RemAssign    rem_assign    %=  }
    vec_op_binary! { $ident { $( $field , )* } impl BitAnd       bitand        &   }
    vec_op_assign! { $ident { $( $field , )* } impl BitAndAssign bitand_assign &=  }
    vec_op_binary! { $ident { $( $field , )* } impl BitOr        bitor         |   }
    vec_op_assign! { $ident { $( $field , )* } impl BitOrAssign  bitor_assign  |=  }
    vec_op_binary! { $ident { $( $field , )* } impl BitXor       bitxor        ^   }
    vec_op_assign! { $ident { $( $field , )* } impl BitXorAssign bitxor_assign ^=  }
    vec_op_binary! { $ident { $( $field , )* } impl Shl          shl           <<  }
    vec_op_assign! { $ident { $( $field , )* } impl ShlAssign    shl_assign    <<= }
    vec_op_binary! { $ident { $( $field , )* } impl Shr          shr           >>  }
    vec_op_assign! { $ident { $( $field , )* } impl ShrAssign    shr_assign    >>= }
    vec_op_unary!  { $ident { $( $field , )* } impl Neg          neg           -   }
    vec_op_unary!  { $ident { $( $field , )* } impl Not          not           !   }

    impl<T> $ident<T> {

        /// Create a new vector by running `f` over the values in this vector.
        ///
        /// This takes ownership of `self`. For a method which doesn't, see
        ///  [`Self::map_ref`].
        #[inline]
        $vis fn map<F>(mut self, mut f : F) -> Self
        where
            F : FnMut(T) -> T
        {
            $( self.$field = f(self.$field) ; )*
            self
        }

        /// Create a new vector by running `f` over the values in this vector.
        ///
        /// This does not take ownership of `self`. For a method which does,
        ///  see [`Self::map`].
        $vis fn map_ref<F>(&self, mut f : F) -> Self
        where
            F : FnMut(&T) -> T
        { Self::new( $( f(&self.$field) , )* ) }

        /// Apply `f` to each element.
        $vis fn map_mut<F>(&mut self, mut f : F) -> ()
        where
            F : FnMut(&mut T)
        { $( f( &mut self.$field ); )* }

        /// Return the elements of the vector as an array.
        $vis fn to_array(&self) -> [T; ${count($field)}]
        where
            T : Copy
        {
            [ $( self.$field , )* ]
        }

    }

    impl<T> $ident<T> {

        /// Returns the dot product of `self` and `rhs`.
        $vis fn dot<U>(self, rhs : Self) -> U
        where
            T : Mul<T, Output = U>,
            U : Add<U, Output = U>
        { strip_plus!( $( + ( self.$field * rhs.$field ) )* ) }

    }

} }
macro_rules! vec_op_binary {(
    $ident:ident { $( $field:ident ),* $(,)? } impl $trait:ident $func:ident $op:tt
) => {
    impl<U : Copy + IsNotVector, T : $trait<U>> $trait<U> for $ident<T> {
        type Output = $ident<<T as $trait<U>>::Output>;
        fn $func(self, rhs : U) -> Self::Output {
            $ident::new( $( self.$field $op rhs , )* )
        }
    }
    impl<U, T : $trait<U>> $trait<$ident<U>> for $ident<T> {
        type Output = $ident<<T as $trait<U>>::Output>;
        fn $func(self, rhs : $ident<U>) -> Self::Output {
            $ident::new( $( self.$field $op rhs.$field , )* )
        }
    }
}}
macro_rules! vec_op_unary {(
    $ident:ident { $( $field:ident ),* $(,)? } impl $trait:ident $func:ident $op:tt
) => {
    impl<T : $trait> $trait for $ident<T> {
        type Output = $ident<<T as $trait>::Output>;
        fn $func(self) -> Self::Output {
            $ident::new( $( $op self.$field , )* )
        }
    }
}}
macro_rules! vec_op_assign {(
    $ident:ident { $( $field:ident ),* $(,)? } impl $trait:ident $func:ident $op:tt
) => {
    impl<U : Copy + IsNotVector, T : $trait<U>> $trait<U> for $ident<T> {
        fn $func(&mut self, rhs : U) -> () {
            $( self.$field $op rhs ; )*
        }
    }
    impl<U, T : $trait<U>> $trait<$ident<U>> for$ident<T> {
        fn $func(&mut self, rhs : $ident<U>) -> () {
            $( self.$field $op rhs.$field ; )*
        }
    }
}}


vec!{ pub Vec2 { x, y } }
vec!{ pub Vec3 { x, y, z } }
