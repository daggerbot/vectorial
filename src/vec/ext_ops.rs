/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use ext_ops::{
    SaturatingAdd,
    SaturatingMul,
    SaturatingNeg,
    SaturatingSub,
    TryAdd,
    TryDiv,
    TryMul,
    TryNeg,
    TrySub,
    WrappingAdd,
    WrappingMul,
    WrappingNeg,
    WrappingSub,
};

use crate::vec::{Vec2, Vec3, Vec4};

/// Implements unary operators for vector types.
macro_rules! impl_unary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self) -> Self::Output {
                $vec { $($field: $trait::$fn(self.$field)),* }
            }
        }

        impl<'a, T> $trait for &'a $vec<T>
        where &'a T: $trait
        {
            type Output = $vec<<&'a T as $trait>::Output>;

            fn $fn(self) -> Self::Output {
                $vec { $($field: $trait::$fn(&self.$field)),* }
            }
        }
    )* };
}

/// Implements vector-scalar binary operators.
macro_rules! impl_scalar_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait<T> for $vec<T>
        where T: Copy + $trait
        {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                $vec { $($field: $trait::$fn(self.$field, rhs)),* }
            }
        }

        impl<'a, T> $trait<T> for &'a $vec<T>
        where T: Copy,
              &'a T: $trait<T>
        {
            type Output = $vec<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                $vec { $($field: $trait::$fn(&self.$field, rhs)),* }
            }
        }

        impl<'r, T> $trait<&'r T> for $vec<T>
        where T: $trait<&'r T>
        {
            type Output = $vec<<T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r T) -> Self::Output {
                $vec { $($field: $trait::$fn(self.$field, &rhs)),* }
            }
        }

        impl<'a, 'r, T> $trait<&'r T> for &'a $vec<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $vec<<&'a T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r T) -> Self::Output {
                $vec { $($field: $trait::$fn(&self.$field, &rhs)),* }
            }
        }
    )* };
}

/// Implements vector-vector binary operators.
macro_rules! impl_binary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                $vec { $($field: $trait::$fn(self.$field, rhs.$field)),* }
            }
        }

        impl<'a, T> $trait<$vec<T>> for &'a $vec<T>
        where &'a T: $trait<T>
        {
            type Output = $vec<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                $vec { $($field: $trait::$fn(&self.$field, rhs.$field)),* }
            }
        }

        impl<'r, T> $trait<&'r $vec<T>> for $vec<T>
        where T: $trait<&'r T>
        {
            type Output = $vec<<T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r $vec<T>) -> Self::Output {
                $vec { $($field: $trait::$fn(self.$field, &rhs.$field)),* }
            }
        }

        impl<'a, 'r, T> $trait<&'r $vec<T>> for &'a $vec<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $vec<<&'a T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r $vec<T>) -> Self::Output {
                $vec { $($field: $trait::$fn(&self.$field, &rhs.$field)),* }
            }
        }
    )* };
}

/// Implements checked unary operators for vector types.
macro_rules! impl_try_unary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            type Output = $vec<<T as $trait>::Output>;
            type Error = <T as $trait>::Error;

            fn $fn(self) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(self.$field)?),* })
            }
        }

        impl<'a, T> $trait for &'a $vec<T>
        where &'a T: $trait
        {
            type Output = $vec<<&'a T as $trait>::Output>;
            type Error = <&'a T as $trait>::Error;

            fn $fn(self) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(&self.$field)?),* })
            }
        }
    )* };
}

/// Implements checked vector-scalar binary operators.
macro_rules! impl_try_scalar_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait<T> for $vec<T>
        where T: Copy + $trait
        {
            type Output = $vec<<T as $trait>::Output>;
            type Error = <T as $trait>::Error;

            fn $fn(self, rhs: T) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(self.$field, rhs)?),* })
            }
        }

        impl<'a, T> $trait<T> for &'a $vec<T>
        where T: Copy,
              &'a T: $trait<T>
        {
            type Output = $vec<<&'a T as $trait<T>>::Output>;
            type Error = <&'a T as $trait<T>>::Error;

            fn $fn(self, rhs: T) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(&self.$field, rhs)?),* })
            }
        }

        impl<'r, T> $trait<&'r T> for $vec<T>
        where T: $trait<&'r T>
        {
            type Output = $vec<<T as $trait<&'r T>>::Output>;
            type Error = <T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r T) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(self.$field, rhs)?),* })
            }
        }

        impl<'a, 'r, T> $trait<&'r T> for &'a $vec<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $vec<<&'a T as $trait<&'r T>>::Output>;
            type Error = <&'a T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r T) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(&self.$field, rhs)?),* })
            }
        }
    )* };
}

/// Implements checked vector-vector binary operators.
macro_rules! impl_try_binary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            type Output = $vec<<T as $trait>::Output>;
            type Error = <T as $trait>::Error;

            fn $fn(self, rhs: $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(self.$field, rhs.$field)?),* })
            }
        }

        impl<'a, T> $trait<$vec<T>> for &'a $vec<T>
        where &'a T: $trait<T>
        {
            type Output = $vec<<&'a T as $trait<T>>::Output>;
            type Error = <&'a T as $trait<T>>::Error;

            fn $fn(self, rhs: $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(&self.$field, rhs.$field)?),* })
            }
        }

        impl<'r, T> $trait<&'r $vec<T>> for $vec<T>
        where T: $trait<&'r T>
        {
            type Output = $vec<<T as $trait<&'r T>>::Output>;
            type Error = <T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(self.$field, &rhs.$field)?),* })
            }
        }

        impl<'a, 'r, T> $trait<&'r $vec<T>> for &'a $vec<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $vec<<&'a T as $trait<&'r T>>::Output>;
            type Error = <&'a T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($vec { $($field: $trait::$fn(&self.$field, &rhs.$field)?),* })
            }
        }
    )* };
}

/// Implements all relevant `ext-ops` traits for vector types.
macro_rules! impl_all {
    { $(impl $vec:ident($($field:ident),*);)* } => { $(
        impl_unary_ops! {
            impl SaturatingNeg::saturating_neg for $vec($($field),*);
            impl WrappingNeg::wrapping_neg for $vec($($field),*);
        }

        impl_scalar_ops! {
            impl SaturatingMul::saturating_mul for $vec($($field),*);
            impl WrappingMul::wrapping_mul for $vec($($field),*);
        }

        impl_binary_ops! {
            impl SaturatingAdd::saturating_add for $vec($($field),*);
            impl SaturatingMul::saturating_mul for $vec($($field),*);
            impl SaturatingSub::saturating_sub for $vec($($field),*);
            impl WrappingAdd::wrapping_add for $vec($($field),*);
            impl WrappingMul::wrapping_mul for $vec($($field),*);
            impl WrappingSub::wrapping_sub for $vec($($field),*);
        }

        impl_try_unary_ops! {
            impl TryNeg::try_neg for $vec($($field),*);
        }

        impl_try_scalar_ops! {
            impl TryDiv::try_div for $vec($($field),*);
            impl TryMul::try_mul for $vec($($field),*);
        }

        impl_try_binary_ops! {
            impl TryAdd::try_add for $vec($($field),*);
            impl TryDiv::try_div for $vec($($field),*);
            impl TryMul::try_mul for $vec($($field),*);
            impl TrySub::try_sub for $vec($($field),*);
        }
    )* };
}

impl_all! {
    impl Vec2(x, y);
    impl Vec3(x, y, z);
    impl Vec4(x, y, z, w);
}
