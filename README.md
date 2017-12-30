# Jsonxf

A JSON transformer, written in Rust.

Provides fast pretty-printing and minimizing of JSON-encoded strings
and streams.

Includes a command-line tool as well as a Rust library.

## Installation

    cargo install jsonxf

## Command-line Example

    jsonxf <foo.json >foo-pretty.json  # pretty-print
    jsonxf -m <foo.json >foo-min.json  # minimize

Run `jsonxf -h` to see configuration options.

## Rust Example

In your `Cargo.toml`:

```
[dependencies]
jsonxf = "0.5"
```

In your code:

```rust
extern crate jsonxf;
let ugly_json = "{\"hello\":\"world\"}";
let pretty_json = jsonxf::pretty_print(ugly_json, "  ").unwrap();
assert_eq!(pretty_json, "{\n  \"hello\": \"world\"\n}\n");
```

## Authorship and License

Copyright 2017, Pete Gamache.

Jsonxf is released under the MIT License.

