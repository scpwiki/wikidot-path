/*
 * lib.rs
 *
 * wikidot-path - Library to parse Wikidot-like paths.
 * Copyright (c) 2019 Ammon Smith
 *
 * wikidot-normalize is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

#![deny(missing_debug_implementations, missing_docs)]

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
extern crate log;
extern crate wikidot_normalize;

mod parse;

#[cfg(test)]
mod test;
