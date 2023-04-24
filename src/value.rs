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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        const SPECIAL_VALUES: [(&str, ArgumentValue); 5] = [
            ("", ArgumentValue::Null),
            ("t", ArgumentValue::Boolean(true)),
            ("f", ArgumentValue::Boolean(false)),
            ("true", ArgumentValue::Boolean(true)),
            ("false", ArgumentValue::Boolean(false)),
        ];

        for (name, result) in &SPECIAL_VALUES {
            if name.eq_ignore_ascii_case(value) {
                return *result;
            }
        }

        match value.parse::<i32>() {
            Ok(int) => ArgumentValue::Integer(int),
            Err(_) => ArgumentValue::String(value),
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
