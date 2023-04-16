/*
 * value.rs
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

/// A type for possible values an argument key could have.
///
/// Those consuming values should attempt to be flexible when
/// accepting values. For instance a truthy key should accept
/// `1`, `true`, and `Null` (as it indicates that they key is
/// present at all) as meaning "true".
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(untagged))]
pub enum OptionValue<'a> {
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

impl<'a> From<Option<&'a str>> for OptionValue<'a> {
    #[inline]
    fn from(value: Option<&'a str>) -> Self {
        match value {
            Some(value) => OptionValue::from(value),
            None => OptionValue::Null,
        }
    }
}

impl<'a> From<&'a str> for OptionValue<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "" => OptionValue::Null,
            "t" | "true" => OptionValue::Boolean(true),
            "f" | "false" => OptionValue::Boolean(false),
            _ => match value.parse::<i32>() {
                Ok(int) => OptionValue::Integer(int),
                Err(_) => OptionValue::String(value),
            },
        }
    }
}

impl From<bool> for OptionValue<'_> {
    #[inline]
    fn from(value: bool) -> Self {
        OptionValue::Boolean(value)
    }
}

impl From<i32> for OptionValue<'_> {
    #[inline]
    fn from(value: i32) -> Self {
        OptionValue::Integer(value)
    }
}

impl From<()> for OptionValue<'_> {
    #[inline]
    fn from(_: ()) -> Self {
        OptionValue::Null
    }
}
