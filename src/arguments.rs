/*
 * arguments.rs
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

use super::schema::ArgumentSchema;
use super::value::ArgumentValue;
use std::collections::HashMap;
use unicase::UniCase;

pub type ArgumentKey<'a> = UniCase<&'a str>;
pub type PageArgumentsMap<'a> = HashMap<ArgumentKey<'a>, (ArgumentValue<'a>, &'a str)>;

/// Represents the set of arguments for a page.
///
/// Within a Wikidot-compatible URL, this is the optional portion
/// *after* a slug. For example:
///
/// * `/scp-1000` -- No page arguments.
/// * `/scp-1000/noredirect/true` -- Page arguments are `/noredirect/true`.
/// * `/scp-1000/noredirect/true/norender/true` -- Page arguments are `/norender/true/noredirect/true`.
///
/// When passed as a string input, the leading `/` character is optional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageArguments<'a>(pub PageArgumentsMap<'a>);

impl<'a> PageArguments<'a> {
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
    pub fn parse(mut path: &'a str, schema: ArgumentSchema) -> Self {
        // Remove leading slash
        if path.starts_with('/') {
            path = &path[1..];
        }

        // Process each section of the options string into keys and values.
        let mut arguments = HashMap::new();
        let mut parts = path.split('/');

        fn process_argument<'a>(
            arguments: &mut PageArgumentsMap<'a>,
            key: &'a str,
            parts: &mut dyn Iterator<Item = &'a str>,
            schema: ArgumentSchema,
        ) {
            let value = parts.next();

            if schema.solo_keys.contains(&key) {
                // If this potentially is a solo key, then check if the next
                // value looks like the next key rather than a value.

                if let Some(value) = value {
                    if schema.valid_keys.contains(&value) {
                        // Yield as solo key
                        //
                        // However if we discard 'value' (really the next pair's key)
                        // we will lose data, so we recursively call this function to
                        // handle it.

                        let key = ArgumentKey::unicode(key);
                        arguments.insert(key, (ArgumentValue::Null, value));
                        process_argument(arguments, value, parts, schema);
                        return;
                    }
                }
            }

            // Otherwise, return as normal key-value pair
            let key = ArgumentKey::unicode(key);
            arguments.insert(key, (ArgumentValue::from(value), value.unwrap_or("")));
        }

        while let Some(key) = parts.next() {
            if !key.is_empty() {
                process_argument(&mut arguments, key, &mut parts, schema);
            }
        }

        PageArguments(arguments)
    }
}
