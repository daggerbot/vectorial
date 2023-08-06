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

use crate::rect::{Rect2, Rect3};
use crate::vec::{Vector2, Vector3};

/// Implements unary operators for rects.
macro_rules! impl_unary_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident;)* } => { $(
        impl<T> $trait for $rect<T>
        where T: $trait
        {
            type Output = $rect<<T as $trait>::Output>;

            fn $fn(self) -> Self::Output {
                $rect($trait::$fn(self.0), $trait::$fn(self.1))
            }
        }

        impl<'a, T> $trait for &'a $rect<T>
        where &'a T: $trait
        {
            type Output = $rect<<&'a T as $trait>::Output>;

            fn $fn(self) -> Self::Output {
                $rect($trait::$fn(&self.0), $trait::$fn(&self.1))
            }
        }
    )* };
}

/// Implements rect-scalar operators.
macro_rules! impl_scalar_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident;)* } => { $(
        impl<T> $trait<T> for $rect<T>
        where T: Copy + $trait
        {
            type Output = $rect<<T as $trait>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                $rect($trait::$fn(self.0, rhs), $trait::$fn(self.1, rhs))
            }
        }

        impl<'a, T> $trait<T> for &'a $rect<T>
        where T: Copy,
              &'a T: $trait<T>
        {
            type Output = $rect<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                $rect($trait::$fn(&self.0, rhs), $trait::$fn(&self.1, rhs))
            }
        }

        impl<'r, T> $trait<&'r T> for $rect<T>
        where T: $trait<&'r T>
        {
            type Output = $rect<<T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r T) -> Self::Output {
                $rect($trait::$fn(self.0, rhs), $trait::$fn(self.1, rhs))
            }
        }

        impl<'a, 'r, T> $trait<&'r T> for &'a $rect<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $rect<<&'a T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r T) -> Self::Output {
                $rect($trait::$fn(&self.0, rhs), $trait::$fn(&self.1, rhs))
            }
        }
    )* };
}

/// Implements rect-vector operators.
macro_rules! impl_vec_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident: $vec:ident;)* } => { $(
        impl<T> $trait<$vec<T>> for $rect<T>
        where T: Copy + $trait
        {
            type Output = $rect<<T as $trait>::Output>;

            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                $rect($trait::$fn(self.0, rhs), $trait::$fn(self.1, rhs))
            }
        }

        impl<'a, T> $trait<$vec<T>> for &'a $rect<T>
        where T: Copy,
              &'a T: $trait<T>
        {
            type Output = $rect<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                $rect($trait::$fn(&self.0, rhs), $trait::$fn(&self.1, rhs))
            }
        }

        impl<'r, T> $trait<&'r $vec<T>> for $rect<T>
        where T: $trait<&'r T>
        {
            type Output = $rect<<T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r $vec<T>) -> Self::Output {
                $rect($trait::$fn(self.0, rhs), $trait::$fn(self.1, rhs))
            }
        }

        impl<'a, 'r, T> $trait<&'r $vec<T>> for &'a $rect<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $rect<<&'a T as $trait<&'r T>>::Output>;

            fn $fn(self, rhs: &'r $vec<T>) -> Self::Output {
                $rect($trait::$fn(&self.0, rhs), $trait::$fn(&self.1, rhs))
            }
        }
    )* };
}

/// Implements checked unary operators for rects.
macro_rules! impl_try_unary_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident;)* } => { $(
        impl<T> $trait for $rect<T>
        where T: $trait
        {
            type Output = $rect<<T as $trait>::Output>;
            type Error = <T as $trait>::Error;

            fn $fn(self) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(self.0)?, $trait::$fn(self.1)?))
            }
        }

        impl<'a, T> $trait for &'a $rect<T>
        where &'a T: $trait
        {
            type Output = $rect<<&'a T as $trait>::Output>;
            type Error = <&'a T as $trait>::Error;

            fn $fn(self) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(&self.0)?, $trait::$fn(&self.1)?))
            }
        }
    )* };
}

/// Implements checked rect-scalar operators.
macro_rules! impl_try_scalar_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident;)* } => { $(
        impl<T> $trait<T> for $rect<T>
        where T: Copy + $trait
        {
            type Output = $rect<<T as $trait>::Output>;
            type Error = <T as $trait>::Error;

            fn $fn(self, rhs: T) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(self.0, rhs)?, $trait::$fn(self.1, rhs)?))
            }
        }

        impl<'a, T> $trait<T> for &'a $rect<T>
        where T: Copy,
              &'a T: $trait<T>
        {
            type Output = $rect<<&'a T as $trait<T>>::Output>;
            type Error = <&'a T as $trait<T>>::Error;

            fn $fn(self, rhs: T) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(&self.0, rhs)?, $trait::$fn(&self.1, rhs)?))
            }
        }

        impl<'r, T> $trait<&'r T> for $rect<T>
        where T: $trait<&'r T>
        {
            type Output = $rect<<T as $trait<&'r T>>::Output>;
            type Error = <T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r T) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(self.0, rhs)?, $trait::$fn(self.1, rhs)?))
            }
        }

        impl<'a, 'r, T> $trait<&'r T> for &'a $rect<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $rect<<&'a T as $trait<&'r T>>::Output>;
            type Error = <&'a T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r T) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(&self.0, rhs)?, $trait::$fn(&self.1, rhs)?))
            }
        }
    )* };
}

/// Implements checked rect-vector operators.
macro_rules! impl_try_vec_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident: $vec:ident;)* } => { $(
        impl<T> $trait<$vec<T>> for $rect<T>
        where T: Copy + $trait
        {
            type Output = $rect<<T as $trait>::Output>;
            type Error = <T as $trait>::Error;

            fn $fn(self, rhs: $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(self.0, rhs)?, $trait::$fn(self.1, rhs)?))
            }
        }

        impl<'a, T> $trait<$vec<T>> for &'a $rect<T>
        where T: Copy,
              &'a T: $trait<T>
        {
            type Output = $rect<<&'a T as $trait<T>>::Output>;
            type Error = <&'a T as $trait<T>>::Error;

            fn $fn(self, rhs: $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(&self.0, rhs)?, $trait::$fn(&self.1, rhs)?))
            }
        }

        impl<'r, T> $trait<&'r $vec<T>> for $rect<T>
        where T: $trait<&'r T>
        {
            type Output = $rect<<T as $trait<&'r T>>::Output>;
            type Error = <T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(self.0, rhs)?, $trait::$fn(self.1, rhs)?))
            }
        }

        impl<'a, 'r, T> $trait<&'r $vec<T>> for &'a $rect<T>
        where &'a T: $trait<&'r T>
        {
            type Output = $rect<<&'a T as $trait<&'r T>>::Output>;
            type Error = <&'a T as $trait<&'r T>>::Error;

            fn $fn(self, rhs: &'r $vec<T>) -> Result<Self::Output, Self::Error> {
                Ok($rect($trait::$fn(&self.0, rhs)?, $trait::$fn(&self.1, rhs)?))
            }
        }
    )* };
}

/// Implements all relevant `ext-ops` traits for rects.
macro_rules! impl_all {
    { $(impl $rect:ident: $vec:ident;)* } => { $(
        impl_unary_ops! {
            impl SaturatingNeg::saturating_neg for $rect;
            impl WrappingNeg::wrapping_neg for $rect;
        }

        impl_scalar_ops! {
            impl SaturatingMul::saturating_mul for $rect;
            impl WrappingMul::wrapping_mul for $rect;
        }

        impl_vec_ops! {
            impl SaturatingAdd::saturating_add for $rect: $vec;
            impl SaturatingMul::saturating_mul for $rect: $vec;
            impl SaturatingSub::saturating_sub for $rect: $vec;
            impl WrappingAdd::wrapping_add for $rect: $vec;
            impl WrappingMul::wrapping_mul for $rect: $vec;
            impl WrappingSub::wrapping_sub for $rect: $vec;
        }

        impl_try_unary_ops! {
            impl TryNeg::try_neg for $rect;
        }

        impl_try_scalar_ops! {
            impl TryDiv::try_div for $rect;
            impl TryMul::try_mul for $rect;
        }

        impl_try_vec_ops! {
            impl TryAdd::try_add for $rect: $vec;
            impl TryDiv::try_div for $rect: $vec;
            impl TryMul::try_mul for $rect: $vec;
            impl TrySub::try_sub for $rect: $vec;
        }
    )* };
}

impl_all! {
    impl Rect2: Vector2;
    impl Rect3: Vector3;
}
