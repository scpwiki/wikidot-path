/*
 * redirect.rs
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

use wikidot_normalize::{is_normal, normalize_decode};

/// Determines if a request with the given path should be redirected or not.
///
/// This is to allow consumers in a web router to redirect requests which are
/// not in Wikidot normal form.
///
/// This also redirects pages in the default category: `_default:page` will be
/// redirected to simply `page`.
pub fn redirect<S: Into<String>>(path: S) -> Option<String> {
    let mut path = path.into();

    debug!("Checking path {} for redirection", &path);
    if is_normal(&path, true) {
        None
    } else {
        normalize_decode(&mut path);

        if path.starts_with("_default") {
            path.replace_range(..9, "");
        }

        Some(path)
    }
}
