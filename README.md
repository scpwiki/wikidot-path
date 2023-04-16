## wikidot-path

<p>
  <a href="https://github.com/scpwiki/wikidot-path/actions?query=workflow%3A%22Rust+CI%22">
    <img src="https://github.com/scpwiki/wikidot-path/workflows/Rust%20CI/badge.svg"
         alt="Rust CI badge">
  </a>

  <a href="https://docs.rs/wikidot-path">
    <img src="https://docs.rs/wikidot-path/badge.svg"
         alt="docs.rs link">
  </a>
</p>

Simple library to provide Wikidot-compatible path parsing.

Wikidot accepts paths in an unusual manner: each argument is submitted as another "directory".

For instance, to access page `scp-xxxx` with options `norender` and `edit`, you would go to `/scp-xxxx/norender/true/edit/true`.

However this also extends its usage in a few minor ways. For instance, options like `edit` only make sense when `true`, but passing that field is always required by Wikidot. Thus the logical URL `/scp-xxxx/edit` doesn't work, despite it being rather clear to a human what the intent is. This library adjusts this by allowing "solo keys" (those without values).

Additionally URL normalization is performed. See the [`wikidot-normalize`](https://crates.io/crates/wikidot-normalize) crate for more information.

Available under the terms of the MIT License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.68.2

```sh
$ cargo build --release
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
