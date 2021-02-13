/*
 * redirect.rs
 *
 * wikidot-path - Library to parse Wikidot-like paths.
 * Copyright (c) 2019-2021 Ammon Smith
 *
 * wikidot-normalize is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

use percent_encoding::percent_decode;
use regex::Regex;
use wikidot_normalize::normalize;

lazy_static! {
    static ref DEFAULT_CATEGORY_REGEX: Regex = Regex::new(r"\b_default:").unwrap();
}

/// Determines if a request with the given path should be redirected or not.
///
/// This is to allow consumers in a web router to redirect requests which are
/// not in Wikidot normal form.
///
/// This also redirects pages in the default category: `_default:page` will be
/// redirected to simply `page`.
///
/// It also decodes any percent codes within the string.
pub fn redirect<S: AsRef<str>>(path: S) -> Option<String> {
    let original_path = path.as_ref();

    debug!("Checking path {}", original_path);

    // Perform percent decoding
    let mut path = {
        let original_path_bytes = original_path.as_bytes();
        let decoded = percent_decode(original_path_bytes).decode_utf8_lossy();
        decoded.into_owned()
    };

    // Normalize path
    normalize(&mut path);

    // Remove _default category
    if let Some(mtch) = DEFAULT_CATEGORY_REGEX.find(&path) {
        let range = mtch.start()..mtch.end();

        path.replace_range(range, "");
    }

    if path == original_path {
        trace!("No redirection needed");

        None
    } else {
        debug!("Redirecting path to {}", path);

        Some(path)
    }
}
