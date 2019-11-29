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

#[test]
fn test_request() {
    macro_rules! check {
        ($path:expr, $expected:expr) => {{
            let page_req = Request::parse($path);
            assert_eq!(page_req, $expected, "Parsed Request doesn't match expected");
        }};
    }

    check!(
        "scp-1000",
        Request {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-1000/edit",
        Request {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-1000/edit/1",
        Request {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => ArgumentValue::from(1) },
        }
    );
    check!(
        "scp-1000/edit/true",
        Request {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => ArgumentValue::from(true) },
        }
    );
    check!(
        "component:image-block",
        Request {
            slug: "image-block",
            categories: vec!["component"],
            arguments: hashmap! {},
        }
    );
    check!(
        "deleted:component:image-block",
        Request {
            slug: "image-block",
            categories: vec!["deleted", "component"],
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1",
        Request {
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1/discuss",
        Request {
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => ArgumentValue::Null },
        }
    );
    check!(
        "fragment:scp-4447-1/discuss/true",
        Request {
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => ArgumentValue::from(true) },
        }
    );
    check!(
        "scp-series-5",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-series-5/norender",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/1",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(1) },
        }
    );
    check!(
        "scp-series-5/norender/true",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(true) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(1), "noredirect" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::from(true) },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect/1",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(1), "noredirect" => ArgumentValue::from(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        Request {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::from(true) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit",
        Request {
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => ArgumentValue::Null },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/1",
        Request {
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => ArgumentValue::from(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/true",
        Request {
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => ArgumentValue::from(true) },
        }
    );
    check!(
        "aaa:bbb:page/noredirect/false/norender/0/true/false",
        Request {
            slug: "page",
            categories: vec!["aaa", "bbb"],
            arguments: hashmap! { "noredirect" => ArgumentValue::from(false), "norender" => ArgumentValue::from(0) },
        }
    );
    check!(
        "aaa:bbb:page/tags/tale/title/A Story/edit",
        Request {
            slug: "page",
            categories: vec!["aaa", "bbb"],
            arguments: hashmap! {
                "tags" => ArgumentValue::from("tale"),
                "title" => ArgumentValue::from("A Story"),
                "edit" => ArgumentValue::Null,
            },
        }
    );
}
