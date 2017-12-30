# Jsonxf

A JSON transformer, written in Rust.

Provides fast pretty-printing and minimizing of JSON-encoded strings
and streams, at the command line or within Rust programs.

Crate docs: https://docs.rs/jsonxf/


## Installation

    cargo install jsonxf


## Command-line Examples

Pretty-print:

    jsonxf <foo.json >foo-pretty.json

Minimize:

    jsonxf -m <foo.json >foo-min.json

Run `jsonxf -h` to see all configuration options.


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


## Performance

Here are some benchmarks comparing Jsonxf's performance to
several of its counterparts: [jsonpp](https://github.com/jmhodges/jsonpp)
and [jq](https://stedolan.github.io/jq/).  `cat` is thrown in as well,
for scale.

Test platform: MBP (early 2013), macOS 10.12.6, 3GHz i7, 8GB RAM.

Pretty-print test, 600MB minimized input (1M objects):

| command   | time (s) | relative time | notes |
|-----------|---------:|--------------:|-------|
| `cat`     |     2.21 |         0.14x | `cat` is a bad pretty-printer |
| `jsonxf`  |    15.58 |            1x | |
| `jsonpp`  |    17.69 |         1.14x | |
| `jq -M .` |    65.86 |         4.22x | |

Minimize test, 850MB pretty-printed input (1M objects):

| command     | time (s) | relative time | notes |
|-------------|---------:|--------------:|-------|
| `cat`       |     3.49 |         0.13x | `cat` is a bad minimizer |
| `jsonxf -m` |    26.98 |            1x | |
| `jsonpp`    |        - |             - | minimizing is not supported |
| `jq -cM .`  |   105.53 |         3.91x | |


## Authorship and License

Copyright 2017-2018, Pete Gamache.

Jsonxf is released under the MIT License.

