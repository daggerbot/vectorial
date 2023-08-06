/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use cgmath::{
    Point2,
    Point3,
};

use crate::vec::{Vector2, Vector3, Vector4};

impl<T> From<Point2<T>> for Vector2<T> {
    fn from(p: Point2<T>) -> Vector2<T> {
        Vector2 { x: p.x, y: p.y }
    }
}

impl<T> From<cgmath::Vector2<T>> for Vector2<T> {
    fn from(v: cgmath::Vector2<T>) -> Vector2<T> {
        Vector2 { x: v.x, y: v.y }
    }
}

impl<T> Into<Point2<T>> for Vector2<T> {
    fn into(self) -> Point2<T> {
        Point2 { x: self.x, y: self.y }
    }
}

impl<T> Into<cgmath::Vector2<T>> for Vector2<T> {
    fn into(self) -> cgmath::Vector2<T> {
        cgmath::Vector2 { x: self.x, y: self.y }
    }
}

//--------------------------------------------------------------------------------------------------

impl<T> From<Point3<T>> for Vector3<T> {
    fn from(p: Point3<T>) -> Vector3<T> {
        Vector3 { x: p.x, y: p.y, z: p.z }
    }
}

impl<T> From<cgmath::Vector3<T>> for Vector3<T> {
    fn from(v: cgmath::Vector3<T>) -> Vector3<T> {
        Vector3 { x: v.x, y: v.y, z: v.z }
    }
}

impl<T> Into<Point3<T>> for Vector3<T> {
    fn into(self) -> Point3<T> {
        Point3 { x: self.x, y: self.y, z: self.z }
    }
}

impl<T> Into<cgmath::Vector3<T>> for Vector3<T> {
    fn into(self) -> cgmath::Vector3<T> {
        cgmath::Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

//--------------------------------------------------------------------------------------------------

impl<T> From<cgmath::Vector4<T>> for Vector4<T> {
    fn from(v: cgmath::Vector4<T>) -> Vector4<T> {
        Vector4 { x: v.x, y: v.y, z: v.z, w: v.w }
    }
}

impl<T> Into<cgmath::Vector4<T>> for Vector4<T> {
    fn into(self) -> cgmath::Vector4<T> {
        cgmath::Vector4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }
}
