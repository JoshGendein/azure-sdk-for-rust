#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "no-default-version")))]
pub use package_preview_2022_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "package-preview-2021-09", not(feature = "no-default-version")))]
pub use package_preview_2021_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-version")))]
pub use package_preview_2021_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2020-11")]
pub mod package_preview_2020_11;
#[cfg(all(feature = "package-preview-2020-11", not(feature = "no-default-version")))]
pub use package_preview_2020_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-07")]
pub mod package_2020_07;
#[cfg(all(feature = "package-2020-07", not(feature = "no-default-version")))]
pub use package_2020_07::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-05-01-preview")]
pub mod package_2019_05_01_preview;
#[cfg(all(feature = "package-2019-05-01-preview", not(feature = "no-default-version")))]
pub use package_2019_05_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
