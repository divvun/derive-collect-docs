# Collect documentation from Rust code

[![Latest version](https://img.shields.io/crates/v/derive-collect-docs.svg)](https://crates.io/crates/derive-collect-docs)
[![Documentation](https://docs.rs/derive-collect-docs/badge.svg)](https://docs.rs/derive-collect-docs)

This little helper library contains a `CollectDocs` derive macro.
Add it to your structs like this:

```rust
/// Very essential type to our application
#[example(
    yaml,
    "
    bar: 42
    baz:
      great_examples_so_far: 2
    "
)]
#[derive(Debug, serde::Deserialize, CollectDocs)]
pub struct Foo {
    /// Amount of pressure to apply
    ///
    /// CAUTION: Will be renamed to appropriate SI unit in next release.
    pub bar: usize,
    /// Description of this Foo
    #[example(
        yaml,
        "
    baz:
      great_examples_so_far: 2
    "
    )]
    pub baz: HashMap<String, i32>,
}
```

This will generate a [AsciiDoc](https://asciidoctor.org/docs/what-is-asciidoc/)
file called `Foo.adoc` (using the struct name).
By default this will be written to `./docs/generated`,
but you change it by setting the `COLLECT_DOCS_TARGET` environment variable.

The rendered document looks like [this](examples/Foo.adoc)
and can be included in your AsciiDoc documentation.

## Examples

Examples are added in separate attributes
that specify the file type followed by the content.
They will be included as code blocks with the heading "Example".

### Tests

Examples in YAML format will also be turned into tests
that exercise that the given content can be parsed by
item the example is on.
For structs, you should give it the full content of the struct,
for fields you neded to also include the field name
in the formatting serde expects (i.e., respecting `#[serde(rename="other_name")]`).

```console
running 2 tests
test test_Foo_baz_example_0 ... ok
test test_Foo_example_0 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

NOTE: You need to add both serde and serde_yaml to your own dependencies.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
