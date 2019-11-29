/*
 * parse.rs
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
    pub slug: &'a str,
    pub categories: Vec<&'a str>,
    pub arguments: HashMap<&'a str, Value<'a>>,
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

                let value = Value::from(parts.next());
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    String(&'a str),
    Integer(i32),
    Boolean(bool),
    Null,
}

cfg_if! {
    if #[cfg(feature = "serde-derive")] {
        use serde::{Serialize, Serializer};

        impl<'a> Serialize for Value<'a> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use self::Value::*;

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

impl<'a> From<Option<&'a str>> for Value<'a> {
    #[inline]
    fn from(value: Option<&'a str>) -> Self {
        match value {
            Some(value) => Value::from(value),
            None => Value::Null,
        }
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "" => Value::Null,
            "true" => Value::Boolean(true),
            "false" => Value::Boolean(false),
            _ => match value.parse::<i32>() {
                Ok(int) => Value::Integer(int),
                Err(_) => Value::String(value),
            },
        }
    }
}

impl From<bool> for Value<'_> {
    #[inline]
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<i32> for Value<'_> {
    #[inline]
    fn from(value: i32) -> Self {
        Value::Integer(value)
    }
}

impl From<()> for Value<'_> {
    #[inline]
    fn from(_: ()) -> Self {
        Value::Null
    }
}
