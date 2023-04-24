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
use std::collections::HashMap;

#[test]
fn test_option_value() {
    macro_rules! check {
        ($input:expr, $expected:expr $(,)?) => {{
            let actual = ArgumentValue::from($input);

            assert_eq!(
                actual, $expected,
                "Actual parsed option value doesn't match expected",
            );
        }};
    }

    check!("", ArgumentValue::Null);
    check!("t", ArgumentValue::Boolean(true));
    check!("f", ArgumentValue::Boolean(false));
    check!("true", ArgumentValue::Boolean(true));
    check!("false", ArgumentValue::Boolean(false));
    check!("T", ArgumentValue::Boolean(true));
    check!("F", ArgumentValue::Boolean(false));
    check!("TRUE", ArgumentValue::Boolean(true));
    check!("False", ArgumentValue::Boolean(false));
    check!("1", ArgumentValue::Integer(1));
    check!("-1", ArgumentValue::Integer(-1));
    check!("9000", ArgumentValue::Integer(9000));
    check!("alpha", ArgumentValue::String("alpha"));
    check!("beta", ArgumentValue::String("beta"));
}

#[test]
fn test_options() {
    const SCHEMA: ArgumentSchema = ArgumentSchema {
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
            // Map to remove original value string
            let actual: HashMap<&str, ArgumentValue> = PageArguments::parse($input, SCHEMA)
                .0
                .into_iter()
                .map(|(key, (value, _))| (key.into_inner(), value))
                .collect();

            assert_eq!(
                actual, $expected,
                "Actual parsed page options don't match expected",
            );
        }};
    }

    macro_rules! o {
        () => {
            ArgumentValue::Null
        };

        ($value:expr $(,)?) => {
            ArgumentValue::from($value)
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
