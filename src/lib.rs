/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Vectors and related mathematic types.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "cgmath")]
extern crate cgmath;

#[cfg(feature = "ext-ops")]
extern crate ext_ops;

#[cfg(feature = "num-complex")]
extern crate num_complex;

#[cfg(feature = "num-traits")]
extern crate num_traits;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod ops;
mod rect;
mod vec;

pub use ops::{Cross, Dot, cross, dot};
pub use rect::{Rect2, Rect3};
pub use vec::{Vector2, Vector3, Vector4, vec2, vec3, vec4};
