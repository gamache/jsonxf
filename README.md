# Jsonxf

A JSON transformer, written in Rust.

Provides fast pretty-printing and minimizing of JSON-encoded strings
and streams, at the command line or within Rust programs.

Crate docs: https://docs.rs/jsonxf/


## Installation

    cargo install jsonxf


## Command-line Examples

Pretty-print a string to the terminal, using two spaces to indent:

    jsonxf -s '{"a": {"b": 2, "c": false}}'

Pretty-print and read a JSON file, using a tab character to indent:

    jsonxf -t $'\t' <foo.json | less

Minimize a file and gzip it:

    jsonxf -m <foo.json | gzip -c >foo-min.json.gz

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
several of its counterparts:
  * [jq](https://stedolan.github.io/jq/), the extremely flexible JSON
    processor.
  * [jsonpp](https://github.com/jmhodges/jsonpp), a JSON pretty-printer
    written in Go.
  * [serdexf](benchmark/serdexf), a trivial example using the
    [serde_json](https://serde.rs/json.html) and
    [serde-transcode](https://serde.rs/transcode.html) libraries.
    This implementation is not complete and is included for library
    comparison only.
  * `cat` is thrown in as well, for scale.

Test platform: MBP (early 2013), macOS 10.12.6, 3GHz i7, 8GB RAM.

See [benchmark.rb](benchmark/benchmark.rb) for testing methodology.

Pretty-print test, 600MB minimized input (1M objects):

| command   | time (s) | relative time | notes |
|-----------|---------:|--------------:|-------|
| `cat`     |     1.49 |          0.2x | `cat` is a bad pretty-printer |
| `jsonxf`  |     7.16 |            1x | |
| `serdexf` |     7.77 |          1.1x | no newlines between objects |
| `jsonpp`  |    18.16 |          2.4x | |
| `jq -M .` |    64.45 |          9.1x | |

Minimize test, 850MB pretty-printed input (1M objects):

| command      | time (s) | relative time | notes |
|--------------|---------:|--------------:|-------|
| `cat`        |     1.46 |          0.3x | `cat` is a bad minimizer |
| `jsonxf -m`  |     4.85 |            1x | |
| `serdexf -m` |     6.33 |          1.3x | |
| `jsonpp`     |        - |             - | minimizing is not supported |
| `jq -cM .`   |   105.53 |           22x | |


## Authorship and License

Copyright 2017-2018, Pete Gamache.

Jsonxf is released under the MIT License.

