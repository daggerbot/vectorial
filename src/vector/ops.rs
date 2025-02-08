/*
 * Copyright (c) 2025 Martin Mills <daggerbot@gmail.com>
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

use crate::vector::{Vector2, Vector3, Vector4};

macro_rules! impl_unary_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*); )* } => { $(
        impl<T> $trait for $vec<T> where T: $trait {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self) -> Self::Output {
                $vec { $($field: self.$field.$fn()),* }
            }
        }
    )* };
}

impl_unary_ops! {
    impl Neg::neg for Vector2(x, y);
    impl Neg::neg for Vector3(x, y, z);
    impl Neg::neg for Vector4(x, y, z, w);
}

macro_rules! impl_scalar_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*); )* } => { $(
        impl<T> $trait<T> for $vec<T> where T: Copy + $trait {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                $vec { $($field: self.$field.$fn(rhs)),* }
            }
        }

        impl<'a, T> $trait<&'a T> for $vec<T> where T: $trait<&'a T> {
            type Output = $vec<<T as $trait<&'a T>>::Output>;

            fn $fn(self, rhs: &'a T) -> Self::Output {
                $vec { $($field: self.$field.$fn(rhs)),* }
            }
        }

        impl<'a, T> $trait<T> for &'a $vec<T> where T: Copy, &'a T: $trait<T> {
            type Output = $vec<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                $vec { $($field: self.$field.$fn(rhs)),* }
            }
        }

        impl<'a, 'b, T> $trait<&'b T> for &'a $vec<T> where &'a T: $trait<&'b T> {
            type Output = $vec<<&'a T as $trait<&'b T>>::Output>;

            fn $fn(self, rhs: &'b T) -> Self::Output {
                $vec { $($field: self.$field.$fn(rhs)),* }
            }
        }
    )* };
}

impl_scalar_ops! {
    impl Div::div for Vector2(x, y);
    impl Div::div for Vector3(x, y, z);
    impl Div::div for Vector4(x, y, z, w);
    impl Mul::mul for Vector2(x, y);
    impl Mul::mul for Vector3(x, y, z);
    impl Mul::mul for Vector4(x, y, z, w);
}

macro_rules! impl_vector_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident: $t:ident),*)[$n:expr]; )* } => { $(
        impl<T> $trait for $vec<T> where T: $trait {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                $vec { $($field: self.$field.$fn(rhs.$field)),* }
            }
        }

        impl<'a, T> $trait<$vec<T>> for &'a $vec<T> where &'a T: $trait<T> {
            type Output = $vec<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                $vec { $($field: self.$field.$fn(rhs.$field)),* }
            }
        }

        impl<'a, T> $trait<&'a $vec<T>> for $vec<T> where T: $trait<&'a T> {
            type Output = $vec<<T as $trait<&'a T>>::Output>;

            fn $fn(self, rhs: &'a $vec<T>) -> Self::Output {
                $vec { $($field: self.$field.$fn(&rhs.$field)),* }
            }
        }

        impl<'a, 'b, T> $trait<&'b $vec<T>> for &'a $vec<T> where &'a T: $trait<&'b T> {
            type Output = $vec<<&'a T as $trait<&'b T>>::Output>;

            fn $fn(self, rhs: &'b $vec<T>) -> Self::Output {
                $vec { $($field: self.$field.$fn(&rhs.$field)),* }
            }
        }

        impl<T> $trait<($($t),*)> for $vec<T> where T: $trait {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self, rhs: ($($t),*)) -> Self::Output {
                let ($($field),*) = rhs;
                $vec { $($field: self.$field.$fn($field)),* }
            }
        }

        impl<'a, T> $trait<($($t),*)> for &'a $vec<T> where &'a T: $trait<T> {
            type Output = $vec<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: ($($t),*)) -> Self::Output {
                let ($($field),*) = rhs;
                $vec { $($field: self.$field.$fn($field)),* }
            }
        }

        impl<T> $trait<[T; $n]> for $vec<T> where T: $trait {
            type Output = $vec<<T as $trait>::Output>;

            fn $fn(self, rhs: [T; $n]) -> Self::Output {
                let [$($field),*] = rhs;
                $vec { $($field: self.$field.$fn($field)),* }
            }
        }

        impl<'a, T> $trait<[T; $n]> for &'a $vec<T> where &'a T: $trait<T> {
            type Output = $vec<<&'a T as $trait<T>>::Output>;

            fn $fn(self, rhs: [T; $n]) -> Self::Output {
                let [$($field),*] = rhs;
                $vec { $($field: self.$field.$fn($field)),* }
            }
        }
    )* };
}

impl_vector_ops! {
    impl Add::add for Vector2(x: T, y: T)[2];
    impl Add::add for Vector3(x: T, y: T, z: T)[3];
    impl Add::add for Vector4(x: T, y: T, z: T, w: T)[4];
    impl Div::div for Vector2(x: T, y: T)[2];
    impl Div::div for Vector3(x: T, y: T, z: T)[3];
    impl Div::div for Vector4(x: T, y: T, z: T, w: T)[4];
    impl Mul::mul for Vector2(x: T, y: T)[2];
    impl Mul::mul for Vector3(x: T, y: T, z: T)[3];
    impl Mul::mul for Vector4(x: T, y: T, z: T, w: T)[4];
    impl Sub::sub for Vector2(x: T, y: T)[2];
    impl Sub::sub for Vector3(x: T, y: T, z: T)[3];
    impl Sub::sub for Vector4(x: T, y: T, z: T, w: T)[4];
}

macro_rules! impl_scalar_assign_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*); )* } => { $(
        impl<T> $trait<T> for $vec<T> where T: Copy + $trait {
            fn $fn(&mut self, rhs: T) {
                $(self.$field.$fn(rhs);)*
            }
        }

        impl<'a, T> $trait<&'a T> for $vec<T> where T: $trait<&'a T> {
            fn $fn(&mut self, rhs: &'a T) {
                $(self.$field.$fn(rhs);)*
            }
        }
    )* };
}

impl_scalar_assign_ops! {
    impl DivAssign::div_assign for Vector2(x, y);
    impl DivAssign::div_assign for Vector3(x, y, z);
    impl DivAssign::div_assign for Vector4(x, y, z, w);
    impl MulAssign::mul_assign for Vector2(x, y);
    impl MulAssign::mul_assign for Vector3(x, y, z);
    impl MulAssign::mul_assign for Vector4(x, y, z, w);
}

macro_rules! impl_vector_assign_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident: $t:ident),*)[$n:expr]; )* } => { $(
        impl<T> $trait for $vec<T> where T: $trait {
            fn $fn(&mut self, rhs: $vec<T>) {
                $(self.$field.$fn(rhs.$field);)*
            }
        }

        impl<'a, T> $trait<&'a $vec<T>> for $vec<T> where T: $trait<&'a T> {
            fn $fn(&mut self, rhs: &'a $vec<T>) {
                $(self.$field.$fn(&rhs.$field);)*
            }
        }

        impl<T> $trait<($($t),*)> for $vec<T> where T: $trait {
            fn $fn(&mut self, rhs: ($($t),*)) {
                let ($($field),*) = rhs;
                $(self.$field.$fn($field);)*
            }
        }

        impl<T> $trait<[T; $n]> for $vec<T> where T: $trait {
            fn $fn(&mut self, rhs: [T; $n]) {
                let [$($field),*] = rhs;
                $(self.$field.$fn($field);)*
            }
        }
    )* };
}

impl_vector_assign_ops! {
    impl AddAssign::add_assign for Vector2(x: T, y: T)[2];
    impl AddAssign::add_assign for Vector3(x: T, y: T, z: T)[3];
    impl AddAssign::add_assign for Vector4(x: T, y: T, z: T, w: T)[4];
    impl DivAssign::div_assign for Vector2(x: T, y: T)[2];
    impl DivAssign::div_assign for Vector3(x: T, y: T, z: T)[3];
    impl DivAssign::div_assign for Vector4(x: T, y: T, z: T, w: T)[4];
    impl MulAssign::mul_assign for Vector2(x: T, y: T)[2];
    impl MulAssign::mul_assign for Vector3(x: T, y: T, z: T)[3];
    impl MulAssign::mul_assign for Vector4(x: T, y: T, z: T, w: T)[4];
    impl SubAssign::sub_assign for Vector2(x: T, y: T)[2];
    impl SubAssign::sub_assign for Vector3(x: T, y: T, z: T)[3];
    impl SubAssign::sub_assign for Vector4(x: T, y: T, z: T, w: T)[4];
}
