//! extensions-rs

pub(crate) mod types;
pub mod validation;

pub use crate::types::{ExtType, Extension};

pub mod ext {
  pub use crate::types::image::Image;
}
