/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/// Computes a cross product.
pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

/// Computes a dot product.
pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// Computes a cross product.
pub fn cross<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> <Lhs as Cross<Rhs>>::Output
where Lhs: Cross<Rhs>
{
    Cross::cross(lhs, rhs)
}

/// Computes a dot product.
pub fn dot<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> <Lhs as Dot<Rhs>>::Output
where Lhs: Dot<Rhs>
{
    Dot::dot(lhs, rhs)
}
