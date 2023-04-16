/*
 * lib.rs
 *
 * wikidot-path - Library to parse Wikidot-like paths.
 * Copyright (c) 2019-2023 Emmie Maeda
 *
 * wikidot-normalize is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

//! A library to provide Wikidot-compatible path parsing.
//!
//! Wikidot accepts paths in an unusual manner: each argument is submitted as another "directory".
//! For instance, the path `/scp-xxxx/norender/true/edit/true` is how you access page `scp-xxxx`
//! with flags "`edit`" and "`norender`" activated.
//!
//! URL normalization is performed when parsing.
//! See the [`wikidot-normalize`](https://crates.io/crates/wikidot-normalize)
//! crate for more information.

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate maplit;
extern crate percent_encoding;

#[cfg(feature = "serde-derive")]
extern crate serde;
extern crate wikidot_normalize;

mod options;
mod redirect;
mod value;

#[cfg(test)]
mod test;

pub use self::options::PageOptions;
pub use self::redirect::redirect;
pub use self::value::OptionValue;

/// A "prelude" for consumers of the `wikidot-path` crate.
///
/// This prelude includes all exports from the crate, and is provided
/// for convenience without requiring programs to do a glob import of
/// the whole crate.
pub mod prelude {
    pub use super::options::PageOptions;
    pub use super::redirect::redirect;
    pub use super::value::OptionValue;
}
