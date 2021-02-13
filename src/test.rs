/*
 * test.rs
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

use crate::prelude::*;

#[test]
fn test_redirect() {
    macro_rules! check {
        ($input:expr, $expected:expr) => {{
            let actual = redirect($input);
            let expected: Option<&str> = $expected;
            let expected = expected.map(String::from);
            assert_eq!(actual, expected, "Redirection didn't match expected");
        }};
    }

    check!("Big Cheese Horace", Some("/big-cheese-horace"));
    check!("Tufto's Proposal", Some("/tufto-s-proposal"));
    check!("SCP-1000", Some("/scp-1000"));
    check!("scp-1000/", Some("/scp-1000"));
    check!("scp-1000", None);
    check!("COMPONENT:image-block", Some("/component:image-block"));
    check!("component:image-block", None);
    check!("_default:scp-1000", Some("/scp-1000"));
    check!("_default:SCP 1000", Some("/scp-1000"));

    check!("/", None);
    check!("/Big Cheese Horace", Some("/big-cheese-horace"));
    check!("/Tufto's Proposal", Some("/tufto-s-proposal"));
    check!("/SCP-1000", Some("/scp-1000"));
    check!("/scp-1000/", Some("/scp-1000"));
    check!("/scp-1000", None);
    check!("/COMPONENT:image-block", Some("/component:image-block"));
    check!("/component:image-block", None);
    check!("/_default:scp-1000", Some("/scp-1000"));
    check!("/_default:SCP 1000", Some("/scp-1000"));

    check!("Big%20Cheese%20Horace", Some("/big-cheese-horace"));
    check!("Tufto%27s%20Proposal", Some("/tufto-s-proposal"));
    check!("SCP%2d1000", Some("/scp-1000"));
    check!("scp%2d1000/", Some("/scp-1000"));
    check!("scp%2d1000", Some("/scp-1000"));
    check!("COMPONENT%3aimage-block", Some("/component:image-block"));
    check!("component%3aimage-block", Some("/component:image-block"));
    check!("_default%3ascp-1000", Some("/scp-1000"));
    check!("_default%3aSCP%201000", Some("/scp-1000"));

    check!("page?", Some("/page"));
    check!("page?q", Some("/page"));
    check!("page?q=test", Some("/page"));
    check!("page?a=1&b=2", Some("/page"));
    check!("Large Reptile?", Some("/large-reptile"));
    check!("Large Reptile?q", Some("/large-reptile"));
    check!("Large Reptile?q=test", Some("/large-reptile"));
    check!("Large Reptile?a=1&b=2", Some("/large-reptile"));
}

#[test]
fn test_request() {
    macro_rules! check {
        ($path:expr, $expected:expr) => {{
            let actual = Request::parse($path);
            assert_eq!(actual, $expected, "Parsed Request doesn't match expected");
        }};
    }

    check!(
        "scp-1000",
        Request {
            slug: "scp-1000",
            category: "_default",
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-1000/edit",
        Request {
            slug: "scp-1000",
            category: "_default",
            arguments: hashmap! { "edit" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-1000/edit/1",
        Request {
            slug: "scp-1000",
            category: "_default",
            arguments: hashmap! { "edit" => ArgumentValue::from(1) },
        }
    );
    check!(
        "scp-1000/edit/true",
        Request {
            slug: "scp-1000",
            category: "_default",
            arguments: hashmap! { "edit" => ArgumentValue::from(true) },
        }
    );
    check!(
        "component:image-block",
        Request {
            slug: "image-block",
            category: "component",
            arguments: hashmap! {},
        }
    );
    check!(
        "deleted:component:image-block",
        Request {
            slug: "component:image-block",
            category: "deleted",
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1",
        Request {
            slug: "scp-4447-1",
            category: "fragment",
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1/discuss",
        Request {
            slug: "scp-4447-1",
            category: "fragment",
            arguments: hashmap! { "discuss" => ArgumentValue::Null },
        }
    );
    check!(
        "fragment:scp-4447-1/discuss/true",
        Request {
            slug: "scp-4447-1",
            category: "fragment",
            arguments: hashmap! { "discuss" => ArgumentValue::from(true) },
        }
    );
    check!(
        "scp-series-5",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-series-5/norender",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/1",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(1) },
        }
    );
    check!(
        "scp-series-5/norender/true",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(true) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(1), "noredirect" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::Null },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::from(true) },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect/1",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(1), "noredirect" => ArgumentValue::from(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        Request {
            slug: "scp-series-5",
            category: "_default",
            arguments: hashmap! { "norender" => ArgumentValue::from(true), "noredirect" => ArgumentValue::from(true) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit",
        Request {
            slug: "page",
            category: "aaa",
            arguments: hashmap! { "edit" => ArgumentValue::Null },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/1",
        Request {
            slug: "page",
            category: "aaa",
            arguments: hashmap! { "edit" => ArgumentValue::from(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/true",
        Request {
            slug: "page",
            category: "aaa",
            arguments: hashmap! { "edit" => ArgumentValue::from(true) },
        }
    );
    check!(
        "aaa:bbb:page/noredirect/false/norender/0/true/false",
        Request {
            slug: "bbb:page",
            category: "aaa",
            arguments: hashmap! { "noredirect" => ArgumentValue::from(false), "norender" => ArgumentValue::from(0) },
        }
    );
    check!(
        "aaa:bbb:page/tags/tale/title/A Story/edit",
        Request {
            slug: "bbb:page",
            category: "aaa",
            arguments: hashmap! {
                "tags" => ArgumentValue::from("tale"),
                "title" => ArgumentValue::from("A Story"),
                "edit" => ArgumentValue::Null,
            },
        }
    );
}
