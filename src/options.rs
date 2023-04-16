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
pub struct PageOptions<'a>(pub HashMap<&'a str, ArgumentValue<'a>>);

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

            let value = ArgumentValue::from(parts.next());
            arguments.insert(key, value);
        }

        PageOptions(arguments)
    }
}

/// A type for possible values an argument key could have.
///
/// Those consuming values should attempt to be flexible when
/// accepting values. For instance a truthy key should accept
/// `1`, `true`, and `Null` (as it indicates that they key is
/// present at all) as meaning "true".
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(untagged))]
pub enum ArgumentValue<'a> {
    /// A string argument value.
    String(&'a str),

    /// An integer argument value.
    Integer(i32),

    /// A boolean argument value.
    Boolean(bool),

    /// No value explicitly passed for this argument.
    /// It notes that the key was included in the mapping.
    Null,
}

impl<'a> From<Option<&'a str>> for ArgumentValue<'a> {
    #[inline]
    fn from(value: Option<&'a str>) -> Self {
        match value {
            Some(value) => ArgumentValue::from(value),
            None => ArgumentValue::Null,
        }
    }
}

impl<'a> From<&'a str> for ArgumentValue<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "" => ArgumentValue::Null,
            "t" | "true" => ArgumentValue::Boolean(true),
            "f" | "false" => ArgumentValue::Boolean(false),
            _ => match value.parse::<i32>() {
                Ok(int) => ArgumentValue::Integer(int),
                Err(_) => ArgumentValue::String(value),
            },
        }
    }
}

impl From<bool> for ArgumentValue<'_> {
    #[inline]
    fn from(value: bool) -> Self {
        ArgumentValue::Boolean(value)
    }
}

impl From<i32> for ArgumentValue<'_> {
    #[inline]
    fn from(value: i32) -> Self {
        ArgumentValue::Integer(value)
    }
}

impl From<()> for ArgumentValue<'_> {
    #[inline]
    fn from(_: ()) -> Self {
        ArgumentValue::Null
    }
}
