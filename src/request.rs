/*
 * request.rs
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

use std::collections::HashMap;

lazy_static! {
    static ref EMPTY_REQUEST: Request<'static> = Request {
        slug: "",
        categories: vec![],
        arguments: hashmap!{},
    };
}

/// Represents a request for a page.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize))]
pub struct Request<'a> {
    /// The slug, or URL identifier of a page.
    ///
    /// For a page like "SCP-1000" this will be `scp-1000`.
    pub slug: &'a str,

    /// A list of categories this page is in, in order of appearance.
    ///
    /// An empty list indicates that no categories were specified,
    /// that is, no colons were present in the path.
    /// By convention any pages like this are considered to be in
    /// the `_default` category.
    pub categories: Vec<&'a str>,

    /// What arguments were passed into the request, as a mapping of
    /// key to value.
    pub arguments: HashMap<&'a str, ArgumentValue<'a>>,
}

impl<'a> Request<'a> {
    /// Parses a path to extract the slug, categories, and arguments.
    /// Makes a best-effort match if the path is not in normal form.
    ///
    /// Returns `None` if invalid.
    pub fn parse(mut path: &'a str) -> Self {
        if path.starts_with('/') {
            trace!("Removing leading slash");
            path = &path[1..];
        }

        // Create part iterator and get slug
        let mut parts = path.split('/');
        let slug = match parts.next() {
            Some(slug) => slug,
            None => {
                trace!("No slug found, returning empty request");

                return EMPTY_REQUEST.clone();
            }
        };

        // Get all page categories
        let (slug, categories) = {
            let mut categories: Vec<_> = slug.split(':').collect();
            let slug = match categories.pop() {
                Some(slug) => slug,
                None => {
                    trace!("Empty categories list, returning empty request");

                    return EMPTY_REQUEST.clone();
                }
            };

            (slug, categories)
        };

        // Parse out Wikidot arguments
        //
        // This algorithm is compatible with the /KEY/true format,
        // but also allowing the more sensible /KEY for options
        // where a 'false' value doesn't make sense, like 'norender' or 'edit'.
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
            categories,
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
            "true" => ArgumentValue::Boolean(true),
            "false" => ArgumentValue::Boolean(false),
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
