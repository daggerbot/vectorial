/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#[cfg(feature = "cgmath")]
mod cgmath;

#[cfg(feature = "ext-ops")]
mod ext_ops;

#[cfg(feature = "num-traits")]
mod num_traits;

use core::fmt::{Display, Formatter};
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

use crate::ops::{Cross, Dot};

/// 2-dimensional vector type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str("(")?;
        Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.y, f)?;
        f.write_str(")")
    }
}

#[cfg(feature = "num-complex")]
impl<T> From<num_complex::Complex<T>> for Vec2<T> {
    fn from(c: num_complex::Complex<T>) -> Vec2<T> {
        Vec2 { x: c.re, y: c.im }
    }
}

#[cfg(feature = "num-complex")]
impl<T> Into<num_complex::Complex<T>> for Vec2<T> {
    fn into(self) -> num_complex::Complex<T> {
        num_complex::Complex { re: self.x, im: self.y }
    }
}

/// 3-dimensional vector type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Cross for Vec3<T>
where T: Copy + Mul,
      <T as Mul>::Output: Sub
{
    type Output = Vec3<<<T as Mul>::Output as Sub>::Output>;

    fn cross(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<'a, T> Cross<Vec3<T>> for &'a Vec3<T>
where T: Copy,
      &'a T: Mul<T>,
      <&'a T as Mul<T>>::Output: Sub
{
    type Output = Vec3<<<&'a T as Mul<T>>::Output as Sub>::Output>;

    fn cross(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: &self.y * rhs.z - &self.z * rhs.y,
            y: &self.z * rhs.x - &self.x * rhs.z,
            z: &self.x * rhs.y - &self.y * rhs.x,
        }
    }
}

impl<'r, T> Cross<&'r Vec3<T>> for Vec3<T>
where T: Copy + Mul<&'r T>,
      <T as Mul<&'r T>>::Output: Sub
{
    type Output = Vec3<<<T as Mul<&'r T>>::Output as Sub>::Output>;

    fn cross(self, rhs: &'r Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.y * &rhs.z - self.z * &rhs.y,
            y: self.z * &rhs.x - self.x * &rhs.z,
            z: self.x * &rhs.y - self.y * &rhs.x,
        }
    }
}

impl<'a, 'r, T> Cross<&'r Vec3<T>> for &'a Vec3<T>
where &'a T: Mul<&'r T>,
      <&'a T as Mul<&'r T>>::Output: Sub
{
    type Output = Vec3<<<&'a T as Mul<&'r T>>::Output as Sub>::Output>;

    fn cross(self, rhs: &'r Vec3<T>) -> Self::Output {
        Vec3 {
            x: &self.y * &rhs.z - &self.z * &rhs.y,
            y: &self.z * &rhs.x - &self.x * &rhs.z,
            z: &self.x * &rhs.y - &self.y * &rhs.x,
        }
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str("(")?;
        Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.y, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.z, f)?;
        f.write_str(")")
    }
}

/// 4-dimensional vector type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Display> Display for Vec4<T> {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str("(")?;
        Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.y, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.z, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.w, f)?;
        f.write_str(")")
    }
}

/// Shorthand constructor for [Vec2].
pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

/// Shorthand constructor for [Vec3].
pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

/// Shorthand constructor for [Vec4].
pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

//--------------------------------------------------------------------------------------------------

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

/// Implements vector-scalar binary assignment operators.
macro_rules! impl_scalar_assign_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait<T> for $vec<T>
        where T: Copy + $trait
        {
            fn $fn(&mut self, rhs: T) {
                $($trait::$fn(&mut self.$field, rhs);)*
            }
        }

        impl<'r, T> $trait<&'r T> for $vec<T>
        where T: $trait<&'r T>
        {
            fn $fn(&mut self, rhs: &'r T) {
                $($trait::$fn(&mut self.$field, rhs);)*
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

/// Implements vector-vector binary assignment operators.
macro_rules! impl_binary_assign_ops {
    { $(impl $trait:ident::$fn:ident for $vec:ident($($field:ident),*);)* } => { $(
        impl<T> $trait for $vec<T>
        where T: $trait
        {
            fn $fn(&mut self, rhs: $vec<T>) {
                $($trait::$fn(&mut self.$field, rhs.$field);)*
            }
        }

        impl<'r, T> $trait<&'r $vec<T>> for $vec<T>
        where T: $trait<&'r T>
        {
            fn $fn(&mut self, rhs: &'r $vec<T>) {
                $($trait::$fn(&mut self.$field, &rhs.$field);)*
            }
        }
    )* };
}

/// Implements common functions and traits for vector types.
macro_rules! impl_all {
    { $(impl $vec:ident($($field:ident: $t:ident),*; $n:expr);)* } => { $(
        impl<T> $vec<T> {
            /// Converts the vector's fields into another type.
            pub fn convert<U>(self) -> $vec<U>
            where T: Into<U>
            {
                $vec { $($field: self.$field.into()),* }
            }

            /// Constructs a new vector.
            pub const fn new($($field: $t),*) -> $vec<T> {
                $vec { $($field),* }
            }

            /// Gets the product of the vector's scalar components.
            pub fn product(self) -> T
            where T: Mul<Output = T>
            {
                // Implementation looks ugly but optimizes to the same machine code as x * y * ...
                let mut a = None;
                $(a = match a {
                    None => Some(self.$field),
                    Some(a) => Some(a * self.$field),
                };)*
                a.unwrap()
            }

            /// Converts the vector's fields into another type.
            pub fn ref_convert<'a, U>(&'a self) -> $vec<U>
            where &'a T: Into<U>
            {
                $vec { $($field: (&self.$field).into()),* }
            }

            /// Gets the sum of the vector's scalar components.
            pub fn sum(self) -> T
            where T: Add<Output = T>
            {
                // Implementation looks ugly but optimizes to the same machine code as x + y + ...
                let mut a = None;
                $(a = match a {
                    None => Some(self.$field),
                    Some(a) => Some(a + self.$field),
                };)*
                a.unwrap()
            }

            /// Attempts to convert the vector's fields into another type. On failure, this returns
            /// the first error that occurred.
            pub fn try_convert<U>(self) -> Result<$vec<U>, <T as TryInto<U>>::Error>
            where T: TryInto<U>
            {
                Ok($vec { $($field: self.$field.try_into()?),* })
            }

            /// Attempts to convert the vector's fields into another type. On failure, this returns
            /// the first error that occurred.
            pub fn try_ref_convert<'a, U>(&'a self) -> Result<$vec<U>, <&'a T as TryInto<U>>::Error>
            where &'a T: TryInto<U>
            {
                Ok($vec { $($field: (&self.$field).try_into()?),* })
            }
        }

        impl<T> Dot for $vec<T>
        where T: Mul,
              <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        {
            type Output = <T as Mul>::Output;

            fn dot(self, rhs: $vec<T>) -> Self::Output {
                (self * rhs).sum()
            }
        }

        impl<'a, T> Dot<$vec<T>> for &'a $vec<T>
        where &'a T: Mul<T>,
              <&'a T as Mul<T>>::Output: Add<Output = <&'a T as Mul<T>>::Output>
        {
            type Output = <&'a T as Mul<T>>::Output;

            fn dot(self, rhs: $vec<T>) -> Self::Output {
                (self * rhs).sum()
            }
        }

        impl<'r, T> Dot<&'r $vec<T>> for $vec<T>
        where T: Mul<&'r T>,
              <T as Mul<&'r T>>::Output: Add<Output = <T as Mul<&'r T>>::Output>
        {
            type Output = <T as Mul<&'r T>>::Output;

            fn dot(self, rhs: &'r $vec<T>) -> Self::Output {
                (self * rhs).sum()
            }
        }

        impl<'a, 'r, T> Dot<&'r $vec<T>> for &'a $vec<T>
        where &'a T: Mul<&'r T>,
              <&'a T as Mul<&'r T>>::Output: Add<Output = <&'a T as Mul<&'r T>>::Output>
        {
            type Output = <&'a T as Mul<&'r T>>::Output;

            fn dot(self, rhs: &'r $vec<T>) -> Self::Output {
                (self * rhs).sum()
            }
        }

        impl<T> From<($($t),*)> for $vec<T> {
            fn from(t: ($($t),*)) -> $vec<T> {
                let ($($field),*) = t;
                $vec { $($field),* }
            }
        }

        impl<T> From<[T; $n]> for $vec<T> {
            fn from(a: [T; $n]) -> $vec<T> {
                let mut iter = a.into_iter();
                $(let $field = iter.next().unwrap();)*
                $vec { $($field),* }
            }
        }

        impl<T> Into<($($t),*)> for $vec<T> {
            fn into(self) -> ($($t),*) {
                ($(self.$field),*)
            }
        }

        impl<T> Into<[T; $n]> for $vec<T> {
            fn into(self) -> [T; $n] {
                [$(self.$field),*]
            }
        }

        impl_unary_ops! {
            impl Neg::neg for $vec($($field),*);
        }

        impl_scalar_ops! {
            impl Div::div for $vec($($field),*);
            impl Mul::mul for $vec($($field),*);
        }

        impl_scalar_assign_ops! {
            impl DivAssign::div_assign for $vec($($field),*);
            impl MulAssign::mul_assign for $vec($($field),*);
        }

        impl_binary_ops! {
            impl Add::add for $vec($($field),*);
            impl Div::div for $vec($($field),*);
            impl Mul::mul for $vec($($field),*);
            impl Sub::sub for $vec($($field),*);
        }

        impl_binary_assign_ops! {
            impl AddAssign::add_assign for $vec($($field),*);
            impl DivAssign::div_assign for $vec($($field),*);
            impl MulAssign::mul_assign for $vec($($field),*);
            impl SubAssign::sub_assign for $vec($($field),*);
        }
    )* };
}

impl_all! {
    impl Vec2(x: T, y: T; 2);
    impl Vec3(x: T, y: T, z: T; 3);
    impl Vec4(x: T, y: T, z: T, w: T; 4);
}
