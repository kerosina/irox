// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;
extern crate core;

pub use primitives::*;
pub use util::*;

#[macro_use]
pub mod ansi_colors;
pub mod arrays;
#[macro_use]
pub mod assert;
pub mod codec;
#[macro_use]
pub mod fmt;
pub mod hex;
pub mod iterators;
pub mod options;
pub mod packetio;
pub mod random;
cfg_feature_std! {
    pub mod read;
}
cfg_feature_std! {
    pub mod sync;
}
cfg_feature_alloc! {
    pub mod vec;
}

pub mod errors;
pub mod fs;
pub mod hash;
mod primitives;

pub mod buf;
mod util;
#[macro_use]
pub mod macros;
