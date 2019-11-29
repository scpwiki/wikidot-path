## wikidot-path
[![Build Status](https://travis-ci.org/Nu-SCPTheme/wikidot-path.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/wikidot-path)

Simple library to provide Wikidot-compatible path parsing.

Wikidot accepts paths in an unusual manner: each argument is submitted as another "directory".

For instance, to access page `scp-xxxx` with options `norender` and `edit`, you would go to `/scp-xxxx/norender/true/edit/true`.

However this also extends its usage in a few minor ways. For instance, options like `edit` only make sense when `true`, but passing that field is always required by Wikidot. Thus the logical URL `/scp-xxxx/edit` doesn't work, despite it being rather clear to a human what the intent is. This library adjusts this by allowing "dangling keys" (those without values).

Additionally URL normalization is performed. See the [`wikidot-normalize`](https://crates.io/crates/wikidot-normalize) crate for more information.

An example would be how the path `/component:image-block/noredirect/true/tags/template` is parsed:

```rust
Request {
    slug: "image-block",
    categories: vec!["component"],
    arguments: hashmap! {
        "noredirect" -> true,
        "tags" -> "template",
    },
}
```

Available under the terms of the MIT License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.39.0

```sh
$ cargo build --release
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
