/*
 * test.rs
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

use crate::prelude::*;

#[test]
fn test_redirect() {
    macro_rules! check {
        ($input:expr, $expected:expr $(,)?) => {{
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
fn test_option_value() {
    macro_rules! check {
        ($input:expr, $expected:expr $(,)?) => {{
            let actual = OptionValue::from($input);

            assert_eq!(
                actual, $expected,
                "Actual parsed option value doesn't match expected",
            );
        }};
    }

    check!("", OptionValue::Null);
    check!("t", OptionValue::Boolean(true));
    check!("f", OptionValue::Boolean(false));
    check!("true", OptionValue::Boolean(true));
    check!("false", OptionValue::Boolean(false));
    check!("T", OptionValue::Boolean(true));
    check!("F", OptionValue::Boolean(false));
    check!("TRUE", OptionValue::Boolean(true));
    check!("False", OptionValue::Boolean(false));
    check!("1", OptionValue::Integer(1));
    check!("-1", OptionValue::Integer(-1));
    check!("9000", OptionValue::Integer(9000));
    check!("alpha", OptionValue::String("alpha"));
    check!("beta", OptionValue::String("beta"));
}

#[test]
fn test_options() {
    const SCHEMA: OptionSchema = OptionSchema {
        valid_keys: &[
            "edit",
            "comments",
            "noredirect",
            "norender",
            "offset",
            "tags",
            "title",
        ],
        solo_keys: &["edit", "comments", "noredirect", "norender"],
    };

    macro_rules! check {
        ($input:expr, $expected:expr $(,)?) => {{
            let actual = PageOptions::parse($input, SCHEMA);

            assert_eq!(
                actual.0, $expected,
                "Actual parsed page options don't match expected",
            );
        }};
    }

    macro_rules! o {
        () => {
            OptionValue::Null
        };

        ($value:expr $(,)?) => {
            OptionValue::from($value)
        };
    }

    check!("", hashmap! {});
    check!("offset/4", hashmap! {"offset" => o!(4)});
    check!("norender/true", hashmap! {"norender" => o!(true)});
    check!("norender/1", hashmap! {"norender" => o!(1)});
    check!("norender", hashmap! {"norender" => o!()});
    check!(
        "norender/t/noredirect",
        hashmap! {"norender" => o!(true), "noredirect" => o!()},
    );
    check!(
        "norender/noredirect/t",
        hashmap! {"norender" => o!(), "noredirect" => o!(true)},
    );
    check!(
        "offset/4/norender/noredirect/",
        hashmap! {"offset" => o!(4), "norender" => o!(), "noredirect" => o!()},
    );
    check!(
        "edit/title/Foo",
        hashmap! {"edit" => o!(), "title" => o!("Foo")},
    );
    check!(
        "edit/T/tags/tale/title/My Tale",
        hashmap! {"edit" => o!(true), "tags" => o!("tale"), "title" => o!("My Tale")},
    );

    check!("/", hashmap! {});
    check!("/offset/2", hashmap! {"offset" => o!(2)});
    check!("/edit/true", hashmap! {"edit" => o!(true)});
}
