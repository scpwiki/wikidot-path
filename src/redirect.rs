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

use regex::Regex;
use wikidot_normalize::{is_normal, normalize_decode};

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
pub fn redirect<S: Into<String>>(path: S) -> Option<String> {
    let mut path = path.into();
    let mut modified = false;

    debug!("Checking path {}", path);

    // Normalize path
    if !is_normal(&path, true) {
        normalize_decode(&mut path);
        modified = true;
    }

    // Remove _default category
    if let Some(mtch) = DEFAULT_CATEGORY_REGEX.find(&path) {
        let range = mtch.start()..mtch.end();

        path.replace_range(range, "");
        modified = true;
    }

    if modified {
        debug!("Redirecting path to {}", path);

        Some(path)
    } else {
        trace!("No redirection needed");

        None
    }
}
