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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Request<'a> {
    pub slug: &'a str,
    pub categories: Vec<&'a str>,
    pub arguments: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub enum Value<'a> {
    String(&'a str),
    Integer(i32),
    Boolean(bool),
    Null,
}
