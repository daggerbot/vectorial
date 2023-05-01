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
    Vector2,
    Vector3,
    Vector4,
};

use crate::vec::{Vec2, Vec3, Vec4};

impl<T> From<Point2<T>> for Vec2<T> {
    fn from(p: Point2<T>) -> Vec2<T> {
        Vec2 { x: p.x, y: p.y }
    }
}

impl<T> From<Vector2<T>> for Vec2<T> {
    fn from(v: Vector2<T>) -> Vec2<T> {
        Vec2 { x: v.x, y: v.y }
    }
}

impl<T> Into<Point2<T>> for Vec2<T> {
    fn into(self) -> Point2<T> {
        Point2 { x: self.x, y: self.y }
    }
}

impl<T> Into<Vector2<T>> for Vec2<T> {
    fn into(self) -> Vector2<T> {
        Vector2 { x: self.x, y: self.y }
    }
}

//--------------------------------------------------------------------------------------------------

impl<T> From<Point3<T>> for Vec3<T> {
    fn from(p: Point3<T>) -> Vec3<T> {
        Vec3 { x: p.x, y: p.y, z: p.z }
    }
}

impl<T> From<Vector3<T>> for Vec3<T> {
    fn from(v: Vector3<T>) -> Vec3<T> {
        Vec3 { x: v.x, y: v.y, z: v.z }
    }
}

impl<T> Into<Point3<T>> for Vec3<T> {
    fn into(self) -> Point3<T> {
        Point3 { x: self.x, y: self.y, z: self.z }
    }
}

impl<T> Into<Vector3<T>> for Vec3<T> {
    fn into(self) -> Vector3<T> {
        Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

//--------------------------------------------------------------------------------------------------

impl<T> From<Vector4<T>> for Vec4<T> {
    fn from(v: Vector4<T>) -> Vec4<T> {
        Vec4 { x: v.x, y: v.y, z: v.z, w: v.w }
    }
}

impl<T> Into<Vector4<T>> for Vec4<T> {
    fn into(self) -> Vector4<T> {
        Vector4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }
}
