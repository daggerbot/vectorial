/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#[cfg(feature = "ext-ops")]
mod ext_ops;

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

use crate::vec::{Vector2, Vector3};

/// 2-dimensional, axis-aligned rectangle structure defined as two opposite points.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect2<T>(pub Vector2<T>, pub Vector2<T>);

/// 3-dimensional, axis-aligned rectangular prism structure defined as two opposite points.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect3<T>(pub Vector3<T>, pub Vector3<T>);

impl<T> Rect3<T> {
    /// Returns `&self.1.z - &self.0.z`.
    pub fn depth<'a>(&'a self) -> <&'a T as Sub>::Output
    where &'a T: Sub
    {
        &self.1.z - &self.0.z
    }
}

//--------------------------------------------------------------------------------------------------

/// Implements unary operators for rectangles.
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

/// Implements rect-scalar assignment operators.
macro_rules! impl_scalar_assign_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident;)* } => { $(
        impl<T> $trait<T> for $rect<T>
        where T: Copy + $trait
        {
            fn $fn(&mut self, rhs: T) {
                $trait::$fn(&mut self.0, rhs);
                $trait::$fn(&mut self.1, rhs);
            }
        }

        impl<'r, T> $trait<&'r T> for $rect<T>
        where T: $trait<&'r T>
        {
            fn $fn(&mut self, rhs: &'r T) {
                $trait::$fn(&mut self.0, rhs);
                $trait::$fn(&mut self.1, rhs);
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

/// Implements rect-vector assignment operators.
macro_rules! impl_vec_assign_ops {
    { $(impl $trait:ident::$fn:ident for $rect:ident: $vec:ident;)* } => { $(
        impl<T> $trait<$vec<T>> for $rect<T>
        where T: Copy + $trait
        {
            fn $fn(&mut self, rhs: $vec<T>) {
                $trait::$fn(&mut self.0, rhs);
                $trait::$fn(&mut self.1, rhs);
            }
        }

        impl<'r, T> $trait<&'r $vec<T>> for $rect<T>
        where T: $trait<&'r T>
        {
            fn $fn(&mut self, rhs: &'r $vec<T>) {
                $trait::$fn(&mut self.0, rhs);
                $trait::$fn(&mut self.1, rhs);
            }
        }
    )* };
}

/// Implements common functionality for rectangles.
macro_rules! impl_all {
    {
        $(
            impl $rect:ident:
            $vec:ident($($field:ident),*),
            ($($field0:ident: $t0:ident),* | $($field1:ident: $t1:ident),*);
        )* } => { $(
        impl<T> $rect<T> {
            /// Converts the rectangle's scalar components to another type.
            pub fn convert<U>(self) -> $rect<U>
            where T: Into<U>
            {
                $rect(self.0.convert(), self.1.convert())
            }

            /// Returns the smallest possible rectangle that includes `self` and `rhs`, assuming
            /// that both rectangles are positive. Simply returns `self` if either rectangle is not
            /// positive.
            pub fn expand(self, rhs: $rect<T>) -> $rect<T>
            where T: PartialOrd
            {
                if !self.is_partially_positive() || !rhs.is_partially_positive() {
                    return self;
                }

                $rect(
                    $vec { $($field: partial_min(self.0.$field, rhs.0.$field)),* },
                    $vec { $($field: partial_max(self.1.$field, rhs.1.$field)),* })
            }

            /// Returns `&self.1.y - &self.0.y`.
            pub fn height<'a>(&'a self) -> <&'a T as Sub>::Output
            where &'a T: Sub
            {
                &self.1.y - &self.0.y
            }

            /// Returns the intersection of two positive rectangles. Returns `None` if either
            /// rectangle's points are not ordered or if the rectangles do not intersect.
            pub fn intersect(self, rhs: $rect<T>) -> Option<$rect<T>>
            where T: PartialOrd
            {
                if !self.is_partially_positive() || !rhs.is_partially_positive() {
                    return None;
                }

                let intersection = $rect(
                    $vec { $($field: partial_max(self.0.$field, rhs.0.$field)),* },
                    $vec { $($field: partial_min(self.1.$field, rhs.1.$field)),* });

                if intersection.is_partially_positive() {
                    Some(intersection)
                } else {
                    None
                }
            }

            /// Returns true if each field in `self.1` is greater than or equal to the corresponding
            /// field in `self.0`.
            pub fn is_ordered(&self) -> bool
            where T: Ord
            {
                self.is_partially_ordered()
            }

            /// Returns true if each field in `self.1` is greater than or equal to the corresponding
            /// field in `self.0`.
            pub fn is_partially_ordered(&self) -> bool
            where T: PartialOrd
            {
                true $(&& self.1.$field >= self.0.$field)*
            }

            /// Returns true if each field in `self.1` is greater than the corresponding field in
            /// `self.0`.
            pub fn is_partially_positive(&self) -> bool
            where T: PartialOrd
            {
                true $(&& self.1.$field > self.0.$field)*
            }

            /// Returns true if each field in `self.1` is greater than the corresponding field in
            /// `self.0`.
            pub fn is_positive(&self) -> bool
            where T: Ord
            {
                self.is_partially_positive()
            }

            /// Constructs a rectangle from decomposed vectors.
            pub const fn new($($field0: $t0),*, $($field1: $t1),*) -> $rect<T> {
                $rect($vec::new($($field0),*), $vec::new($($field1),*))
            }

            /// Sorts the corresponding fields of `self.0` and `self.1` in ascending order.
            pub fn ordered(self) -> $rect<T>
            where T: Ord
            {
                self.partially_ordered()
            }

            /// Sorts the corresponding fields of `self.0` and `self.1` in ascending order.
            pub fn partially_ordered(self) -> $rect<T>
            where T: PartialOrd
            {
                $(let $field = sort(self.0.$field, self.1.$field);)*
                $rect($vec::new($($field.0),*), $vec::new($($field.1),*))
            }

            /// Converts the rectangle's scalar components to another type.
            pub fn ref_convert<'a, U>(&'a self) -> $rect<U>
            where &'a T: Into<U>
            {
                $rect(self.0.ref_convert(), self.1.ref_convert())
            }

            /// Returns `self.1 - self.0`.
            pub fn size<'a>(&'a self) -> $vec<<&'a T as Sub>::Output>
            where &'a T: Sub
            {
                &self.1 - &self.0
            }

            /// Attempts to convert the rectangle's scalar components to another type.
            pub fn try_convert<U>(self) -> Result<$rect<U>, <T as TryInto<U>>::Error>
            where T: TryInto<U>
            {
                Ok($rect(self.0.try_convert()?, self.1.try_convert()?))
            }

            /// Attempts to convert the rectangle's scalar components to another type.
            pub fn try_ref_convert<'a, U>(&'a self)
                -> Result<$rect<U>, <&'a T as TryInto<U>>::Error>
            where &'a T: TryInto<U>
            {
                Ok($rect(self.0.try_ref_convert()?, self.1.try_ref_convert()?))
            }

            /// Returns `&self.1.x - &self.0.x`.
            pub fn width<'a>(&'a self) -> <&'a T as Sub>::Output
            where &'a T: Sub
            {
                &self.1.x - &self.0.x
            }
        }

        impl<T> From<($($t0),*, $($t1),*)> for $rect<T> {
            fn from(t: ($($t0),*, $($t1),*)) -> $rect<T> {
                let ($($field0),*, $($field1),*) = t;
                $rect($vec::new($($field0),*), $vec::new($($field1),*))
            }
        }

        impl<T> From<($($t1),*)> for $rect<T>
        where T: Default
        {
            fn from(size: ($($t1),*)) -> $rect<T> {
                let ($($field1),*) = size;
                $rect($vec::default(), $vec::new($($field1),*))
            }
        }

        impl<T> From<$vec<T>> for $rect<T>
        where T: Default
        {
            fn from(size: $vec<T>) -> $rect<T> {
                $rect($vec::default(), size)
            }
        }

        impl_unary_ops! {
            impl Neg::neg for $rect;
        }

        impl_scalar_ops! {
            impl Div::div for $rect;
            impl Mul::mul for $rect;
        }

        impl_scalar_assign_ops! {
            impl DivAssign::div_assign for $rect;
            impl MulAssign::mul_assign for $rect;
        }

        impl_vec_ops! {
            impl Add::add for $rect: $vec;
            impl Div::div for $rect: $vec;
            impl Mul::mul for $rect: $vec;
            impl Sub::sub for $rect: $vec;
        }

        impl_vec_assign_ops! {
            impl AddAssign::add_assign for $rect: $vec;
            impl DivAssign::div_assign for $rect: $vec;
            impl MulAssign::mul_assign for $rect: $vec;
            impl SubAssign::sub_assign for $rect: $vec;
        }
    )* };
}

impl_all! {
    impl Rect2: Vector2(x, y), (x0: T, y0: T | x1: T, y1: T);
    impl Rect3: Vector3(x, y, z), (x0: T, y0: T, z0: T | x1: T, y1: T, z1: T);
}

//--------------------------------------------------------------------------------------------------

fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if b < a {
        b
    } else {
        a
    }
}

fn sort<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

//--------------------------------------------------------------------------------------------------

#[test]
fn test_rect_intersection() {
    assert_eq!(Rect2::new(0, 1, 80, 81).intersect(Rect2::new(20, 21, 100, 101)),
               Some(Rect2::new(20, 21, 80, 81)));
    assert_eq!(Rect2::new(1, 0, 101, 100).intersect(Rect2::new(21, 20, 81, 80)),
               Some(Rect2::new(21, 20, 81, 80)));
    assert_eq!(Rect2::new(0, 0, 10, 10).intersect(Rect2::new(10, 0, 20, 10)), None);
    assert_eq!(Rect2::new(0, 0, 100, 100).intersect(Rect2::new(50, 50, 50, 50)), None);
    assert_eq!(Rect2::new(0, 0, 100, 100).intersect(Rect2::new(80, 80, 20, 20)), None);
    assert_eq!(Rect2::new(0, 0, 10, 10).intersect(Rect2::new(20, 0, 30, 10)), None);
}

#[test]
fn test_rect_ordering() {
    assert!(Rect2::new(0, 1, 2, 3).is_ordered());
    assert!(Rect2::new(0, 1, 0, 3).is_ordered());
    assert!(!Rect2::new(2, 1, 0, 3).is_ordered());
    assert!(Rect2::new(0, 1, 2, 3).is_positive());
    assert!(!Rect2::new(0, 1, 0, 3).is_positive());
    assert!(!Rect2::new(2, 1, 0, 3).is_positive());
    assert_eq!(Rect2::new(0, 1, 2, 3).ordered(), Rect2::new(0, 1, 2, 3));
    assert_eq!(Rect2::new(2, 1, 0, 3).ordered(), Rect2::new(0, 1, 2, 3));
}
