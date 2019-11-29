/*
 * test.rs
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

use crate::prelude::*;

#[test]
fn test_redirect() {
    macro_rules! check {
        ($input:expr, $expected:expr) => {{
            let path = redirect($input);
            let expected: Option<&str> = $expected;
            let expected = expected.map(|s| str!(s));
            assert_eq!(path, expected, "Redirection didn't match expected");
        }};
    }

    check!("", None);
    check!("Big Cheese Horace", Some("big-cheese-horace"));
    check!("Tufto's Proposal", Some("tufto-s-proposal"));
    check!("SCP-1000", Some("scp-1000"));
    check!("scp-1000", None);

    check!("/", None);
    check!("/Big Cheese Horace", Some("/big-cheese-horace"));
    check!("/Tufto's Proposal", Some("/tufto-s-proposal"));
    check!("/SCP-1000", Some("/scp-1000"));
    check!("/scp-1000", None);
}
