#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2019-12-01")]
pub mod package_2019_12_01;
#[cfg(all(feature = "package-2019-12-01", not(feature = "no-default-version")))]
pub use package_2019_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-06-01-preview")]
pub mod package_2019_06_01_preview;
#[cfg(all(feature = "package-2019-06-01-preview", not(feature = "no-default-version")))]
pub use package_2019_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
