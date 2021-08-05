#![warn(missing_docs)]

//! # autograph
//! A machine learning library for Rust.
//!
//! To use autograph in your crate, add it as a dependency in Cargo.toml:
//!```text
//! [dependencies]
//! autograph = { git = https://github.com/charles-r-earp/autograph }
//!```
//!
//! # Requirements
//! - A device (typically a gpu) with drivers for a supported API:
//!     - Vulkan (All platforms) <https://www.vulkan.org/>
//!     - Metal (MacOS / iOS) <https://developer.apple.com/metal/>
//!     - DX12 (Windows) <https://docs.microsoft.com/windows/win32/directx>

#![cfg_attr(feature = "bench", feature(test))]

#[cfg(feature = "bench")]
extern crate test;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate autograph_derive;

/// Result type.
pub mod result {
    pub use anyhow::Result;
}
/// Error type.
pub mod error {
    pub use anyhow::Error;
}
/// Device level backend.
pub mod device;

mod util;

#[doc(hidden)]
pub mod glsl_shaders;
#[doc(hidden)]
pub mod rust_shaders;

/// Float types.
pub mod float;
/// Scalar types.
pub mod scalar;

/// Linear Algebra.
#[doc(hidden)]
pub mod linalg;
/// Numerical operations.
mod ops;

/// Tensors.
#[cfg(feature = "tensor")]
pub mod tensor;

/// Float Tensors.
#[cfg(feature = "tensor")]
pub mod float_tensor;

/// Datasets.
pub mod dataset;

/// Machine Learning.
pub mod learn;
