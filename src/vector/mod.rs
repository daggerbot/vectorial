/*
 * Copyright (c) 2025 Martin Mills <daggerbot@gmail.com>
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod ops;

/// 2-dimensional vector type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    /// Constructs a vector from its scalar parts.
    pub const fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(tuple: (T, T)) -> Vector2<T> {
        let (x, y) = tuple;
        Vector2 { x, y }
    }
}

impl<T> From<[T; 2]> for Vector2<T> {
    fn from(array: [T; 2]) -> Vector2<T> {
        let [x, y] = array;
        Vector2 { x, y }
    }
}

impl<T> Into<(T, T)> for Vector2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Into<[T; 2]> for Vector2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

/// 3-dimensional vector type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    /// Constructs a vector from its scalar parts.
    pub const fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from(tuple: (T, T, T)) -> Vector3<T> {
        let (x, y, z) = tuple;
        Vector3 { x, y, z }
    }
}

impl<T> From<[T; 3]> for Vector3<T> {
    fn from(array: [T; 3]) -> Vector3<T> {
        let [x, y, z] = array;
        Vector3 { x, y, z }
    }
}

impl<T> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T> Into<[T; 3]> for Vector3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

/// 4-dimensional vector type.
///
/// This is sometimes used to represent homogeneous coordinates in 3D graphics, but unless otherwise
/// specified, this is treated as a 4-dimensional vector with the `w` component treated the same as
/// the others.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    /// Constructs a vector from its scalar parts.
    pub const fn new(x: T, y: T, z: T, w: T) -> Vector4<T> {
        Vector4 { x, y, z, w }
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from(tuple: (T, T, T, T)) -> Vector4<T> {
        let (x, y, z, w) = tuple;
        Vector4 { x, y, z, w }
    }
}

impl<T> From<[T; 4]> for Vector4<T> {
    fn from(array: [T; 4]) -> Vector4<T> {
        let [x, y, z, w] = array;
        Vector4 { x, y, z, w }
    }
}

impl<T> Into<(T, T, T, T)> for Vector4<T> {
    fn into(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T> Into<[T; 4]> for Vector4<T> {
    fn into(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
