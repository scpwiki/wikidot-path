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

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[cfg(feature = "serde-derive")]
#[macro_use]
extern crate serde;

mod arguments;
mod schema;
mod value;

#[cfg(test)]
mod test;

pub use self::arguments::PageArguments;
pub use self::schema::ArgumentSchema;
pub use self::value::ArgumentValue;

/// A "prelude" for consumers of the `wikidot-path` crate.
///
/// This prelude includes all exports from the crate, and is provided
/// for convenience without requiring programs to do a glob import of
/// the whole crate.
pub mod prelude {
    pub use super::arguments::PageArguments;
    pub use super::schema::ArgumentSchema;
    pub use super::value::ArgumentValue;
}
