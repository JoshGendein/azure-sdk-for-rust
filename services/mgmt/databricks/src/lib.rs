#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2018-04-01")]
pub mod package_2018_04_01;
#[cfg(all(feature = "package-2018-04-01", not(feature = "no-default-version")))]
pub use package_2018_04_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
