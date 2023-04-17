/*
 * schema.rs
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

/// Helper structure to assist with argument parsing.
#[derive(Debug, Copy, Clone)]
pub struct ArgumentSchema<'a> {
    /// Describes which keys are valid.
    pub valid_keys: &'a [&'a str],

    /// List of keys which do not require a value.
    pub solo_keys: &'a [&'a str],
}
