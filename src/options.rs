/*
 * options.rs
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

use super::value::OptionValue;
use std::collections::HashMap;

/// Represents the set of options for a page.
///
/// Within a Wikidot-compatible URL, this is the optional portion
/// *after* a slug. For example:
///
/// * `/scp-1000` -- No page options.
/// * `/scp-1000/noredirect/true` -- Page options are `/noredirect/true`.
/// * `/scp-1000/noredirect/true/norender/true` -- Page options are `/norender/true/noredirect/true`.
///
/// When passed as a string input, the leading `/` character is optional.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize))]
pub struct PageOptions<'a>(pub HashMap<&'a str, OptionValue<'a>>);

impl<'a> PageOptions<'a> {
    /// Parse out Wikidot arguments.
    ///
    /// This algorithm is compatible with the `/KEY/true` format,
    /// but also allows a lone `/KEY` for options which are "innately valued",
    /// such as `norender` or `edit`, where adding a `/true` is not very useful.
    ///
    /// This means that for `/KEY1/KEY2/VALUE` where value is not a string
    /// (i.e. null, boolean, or integer),
    ///
    /// If there are duplicate keys, the most recent one takes precedence.
    pub fn parse(mut path: &'a str) -> Self {
        // Remove leading slash
        if path.starts_with('/') {
            path = &path[1..];
        }

        // Process each
        let mut arguments = HashMap::new();
        let mut parts = path.split('/');

        while let Some(key) = parts.next() {
            // If this looks like a value, then skip it.
            if key.is_empty() || key == "true" || key == "false" {
                continue;
            }

            let value = OptionValue::from(parts.next());
            arguments.insert(key, value);
        }

        PageOptions(arguments)
    }
}
