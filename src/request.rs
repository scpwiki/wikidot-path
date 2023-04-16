/*
 * request.rs
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

lazy_static! {
    static ref EMPTY_REQUEST: Request<'static> = Request {
        slug: "",
        category: "_default",
        arguments: hashmap! {},
    };
}

/// Represents a request for a page.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize))]
pub struct Request<'a> {
    /// The slug, or URL identifier of a page, including its category.
    ///
    /// For instance, `scp-1000` or `system:recent-changes`.
    pub slug: &'a str,

    /// The category that this page appears in.
    /// If in the default category, then this string is `_default`.
    pub category: &'a str,

    /// What arguments were passed into the request.
    /// A mapping of key to value.
    pub arguments: HashMap<&'a str, ArgumentValue<'a>>,
}

impl<'a> Request<'a> {
    /// Parses a path to extract the slug, categories, and arguments.
    /// Makes a best-effort match if the path is not in normal form.
    pub fn parse(mut path: &'a str) -> Self {
        if path.starts_with('/') {
            // Remove leading slash
            path = &path[1..];
        }

        // Create part iterator and get slug
        let mut parts = path.split('/');
        let slug = match parts.next() {
            Some(slug) => slug,
            None => {
                // No slug found, return an empty request
                return EMPTY_REQUEST.clone();
            }
        };

        // Get all page categories
        let (slug, category) = match slug.find(':') {
            // We can't use .split_at() because we want to
            // exclude the ':' from appearing in the string.
            Some(idx) => (&slug[idx + 1..], &slug[..idx]),
            None => (slug, "_default"),
        };

        // Parse out Wikidot arguments
        //
        // This algorithm is compatible with the /KEY/true format,
        // but also allowing the more sensible /KEY for options
        // where a 'false' value doesn't make sense, like 'norender' or 'edit'.
        //
        // If there are duplicate keys, the most recent one takes precedence.
        let arguments = {
            let mut arguments = HashMap::new();

            while let Some(key) = parts.next() {
                if key.is_empty() || key == "true" || key == "false" {
                    continue;
                }

                let value = ArgumentValue::from(parts.next());
                arguments.insert(key, value);
            }

            arguments
        };

        Request {
            slug,
            category,
            arguments,
        }
    }
}

/// A type for possible values an argument key could have.
///
/// Those consuming values should attempt to be flexible when
/// accepting values. For instance a truthy key should accept
/// `1`, `true`, and `Null` (as it indicates that they key is
/// present at all) as meaning "true".
#[derive(Debug, Clone, PartialEq, Eq)]
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

cfg_if! {
    if #[cfg(feature = "serde-derive")] {
        use serde::{Serialize, Serializer};

        impl<'a> Serialize for ArgumentValue<'a> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use self::ArgumentValue::*;

                match self {
                    String(value) => serializer.serialize_str(value),
                    Boolean(value) => serializer.serialize_bool(*value),
                    Integer(value) => serializer.serialize_i32(*value),
                    Null => serializer.serialize_unit(),
                }
            }
        }
    }
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
