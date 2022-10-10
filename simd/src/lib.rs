// pathfinder/simd/src/lib.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(pf_rustc_nightly, feature(link_llvm_intrinsics, platform_intrinsics))]
#![cfg_attr(pf_rustc_nightly, feature(simd_ffi, stdsimd))]

//! A minimal SIMD abstraction, usable outside of Pathfinder.

// Use ARM as the default crate implementation.
#[cfg(all(not(feature = "pf-no-simd"), pf_rustc_nightly, target_arch = "aarch64"))]
pub use crate::arm as default;

// Use scalar as the default crate implementation.
#[cfg(any(
    feature = "pf-no-simd",
    not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        all(pf_rustc_nightly, target_arch = "aarch64")
    ))
))]
pub use crate::scalar as default;

// Use x86/x86_64 as the default crate implementation.
#[cfg(all(
    not(feature = "pf-no-simd"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
pub use crate::x86 as default;

mod extras;

#[cfg(all(pf_rustc_nightly, target_arch = "aarch64"))]
pub mod arm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

pub mod scalar;

#[cfg(test)]
mod test;
