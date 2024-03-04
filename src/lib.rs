//! extensions-rs
//!
//! [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/travisbaars/extensions-rs/integrate.yml?branch=main&logo=github)](https://github.com/travisbaars/extensions-rs/actions)
//! [![Crates.io Version](https://img.shields.io/crates/v/extensions-rs)](https://crates.io/crates/extensions-rs)
//! [![docs.rs](https://img.shields.io/docsrs/extensions-rs)](https://docs.rs/extensions-rs)
//!
//! A collection of file extension types in Rust.
//!
//! The idea behind this crate is to give a simple way of handling file extension types.
//!
//! ## Installation
//!
//! Add `extensions-rs` to your project's `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! extensions-rs = "0.2.0"
//! ```
//!
//! Or use `cargo add`:
//!
//! ```bash
//! cargo add extensions-rs
//! ```
//!
//! ## Examples
//!
//! #### Conversion from `Extension` to `str`:
//!
//! ```rust
//! use extensions_rs::Extension;
//! use extensions_rs::ext::Image;
//!
//! assert_eq!("png", Extension::to_str(Extension::Image(Image::ExtPNG)));
//! ```
//!
//! #### Simple conversion, `&str` to `Image` type:
//!
//! ```rust
//! use extensions_rs::ext::Image;
//!
//! assert_eq!(Image::ExtJPG, Image::from("jpg"));
//! ```
//!
//! #### Validate extension:
//!
//! ```
//! use extensions_rs::utils::Validate;
//!
//! # tokio_test::block_on(async {
//! assert_eq!(true, Validate::check_str("jpg").await);
//! # })
//! ```

pub(crate) mod types;
pub(crate) mod validation;

pub use crate::types::{ExtType, Extension};

pub mod ext {
    pub use crate::types::image::Image;
}

pub mod utils {
    pub use crate::validation::Validate;
}
