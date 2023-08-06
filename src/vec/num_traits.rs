/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use num_traits::{
    CheckedAdd,
    CheckedDiv,
    CheckedMul,
    CheckedNeg,
    CheckedSub,
    One,
    SaturatingAdd,
    SaturatingMul,
    SaturatingSub,
    WrappingAdd,
    WrappingMul,
    WrappingNeg,
    WrappingSub,
    Zero,
};

use crate::vec::{Vector2, Vector3, Vector4};

/// Implements unary operator traits for vector types.
macro_rules! impl_unary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            fn $fn(&self) -> $vec<T> {
                $vec { $($field: self.$field.$fn()),* }
            }
        }
    )* };
}

/// Implements binary operator traits for vector types.
macro_rules! impl_binary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            fn $fn(&self, rhs: &$vec<T>) -> $vec<T> {
                $vec { $($field: self.$field.$fn(&rhs.$field)),* }
            }
        }
    )* };
}

/// Implements checked unary operator traits for vector types.
macro_rules! impl_checked_unary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            fn $fn(&self) -> Option<$vec<T>> {
                $(let $field = match self.$field.$fn() {
                    None => return None,
                    Some(n) => n,
                };)*
                Some($vec { $($field),* })
            }
        }
    )* };
}

/// Implements checked binary operator traits for vector types.
macro_rules! impl_checked_binary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            fn $fn(&self, rhs: &$vec<T>) -> Option<$vec<T>> {
                $(let $field = match self.$field.$fn(&rhs.$field) {
                    None => return None,
                    Some(n) => n,
                };)*
                Some($vec { $($field),* })
            }
        }
    )* };
}

/// Implements all relevant `num-traits` traits for vector types.
macro_rules! impl_all {
    { $(impl $vec:ident($($field:ident),*);)* } => { $(
        impl<T> One for $vec<T>
        where T: One + PartialEq
        {
            fn is_one(&self) -> bool where $vec<T>: PartialEq {
                true $(&& self.$field.is_one())*
            }

            fn one() -> $vec<T> {
                $vec { $($field: One::one()),* }
            }

            fn set_one(&mut self) {
                $(self.$field.set_one();)*
            }
        }

        impl<T> Zero for $vec<T>
        where T: Zero
        {
            fn is_zero(&self) -> bool {
                true $(&& self.$field.is_zero())*
            }

            fn set_zero(&mut self) {
                $(self.$field.set_zero();)*
            }

            fn zero() -> $vec<T> {
                $vec { $($field: Zero::zero()),* }
            }
        }

        impl_unary_ops! {
            impl WrappingNeg::wrapping_neg for $vec($($field),*);
        }

        impl_binary_ops! {
            impl SaturatingAdd::saturating_add for $vec($($field),*);
            impl SaturatingMul::saturating_mul for $vec($($field),*);
            impl SaturatingSub::saturating_sub for $vec($($field),*);
            impl WrappingAdd::wrapping_add for $vec($($field),*);
            impl WrappingMul::wrapping_mul for $vec($($field),*);
            impl WrappingSub::wrapping_sub for $vec($($field),*);
        }

        impl_checked_unary_ops! {
            impl CheckedNeg::checked_neg for $vec($($field),*);
        }

        impl_checked_binary_ops! {
            impl CheckedAdd::checked_add for $vec($($field),*);
            impl CheckedDiv::checked_div for $vec($($field),*);
            impl CheckedMul::checked_mul for $vec($($field),*);
            impl CheckedSub::checked_sub for $vec($($field),*);
        }
    )* };
}

impl_all! {
    impl Vector2(x, y);
    impl Vector3(x, y, z);
    impl Vector4(x, y, z, w);
}
