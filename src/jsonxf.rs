//! Jsonxf is a JSON transformer, providing fast pretty-printing and
//! minimizing of JSON-encoded data.
//!
//! Jsonxf is built for speed, and does not attempt to perform any
//! input validation whatsoever.  Invalid input may produce strange
//! output.
//!
//! Installing this project via `cargo install` will also install the
//! `jsonxf` command-line tool.  Run `jsonxf -h` to see configuration
//! options.
//!
//! GitHub:
//! <a href="https://github.com/gamache/jsonxf" target="_blank">gamache/jsonxf</a>
//!

use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;

const BUF_SIZE: usize = 1024 * 16;

const C_CR:             u8 = '\r' as u8;
const C_LF:             u8 = '\n' as u8;
const C_TAB:            u8 = '\t' as u8;
const C_SPACE:          u8 = ' ' as u8;

const C_COMMA:          u8 = ',' as u8;
const C_COLON:          u8 = ':' as u8;
const C_QUOTE:          u8 = '"' as u8;
const C_BACKSLASH:      u8 = '\\' as u8;

const C_LEFT_BRACE:     u8 = '{' as u8;
const C_LEFT_BRACKET:   u8 = '[' as u8;
const C_RIGHT_BRACE:    u8 = '}' as u8;
const C_RIGHT_BRACKET:  u8 = ']' as u8;

struct PpState<'a> {
  indent: &'a str,
  depth: usize,

  // string state flags
  in_string: bool,
  in_backslash: bool,

  // required to support the case of empty {} and []
  empty: bool,  // true if !in_string and last non-whitespace was { or [
  wbuf: String, // whitespace buffer
}

struct MinState {
  in_string: bool,
  in_backslash: bool,

  // required so we don't emit a newline at the end of a stream
  depth: usize,
  print_newline: bool,
}


/// Pretty-prints a string of JSON-encoded data.
///
/// Valid inputs produce valid outputs.  However, this function does
/// *not* perform JSON validation, and is not guaranteed to either
/// reject or accept invalid input.
///
/// The `indent` parameter sets the string used to indent pretty-printed
/// output; e.g., `"  "` or `"\t"`.
///
/// # Examples:
///
/// ```
/// assert_eq!(
///   jsonxf::pretty_print("{\"a\":1,\"b\":2}", "  ").unwrap(),
///   "{\n  \"a\": 1,\n  \"b\": 2\n}\n"
/// );
/// assert_eq!(
///   jsonxf::pretty_print("{\"empty\":{},\n\n\n\n\n\"one\":[1]}", "\t").unwrap(),
///   "{\n\t\"empty\": {},\n\t\"one\": [\n\t\t1\n\t]\n}\n"
/// );
/// ```
///
pub fn pretty_print(json_string: &str, indent: &str) -> Result<String, String> {
  let mut input = json_string.as_bytes();
  let mut output: Vec<u8> = vec![];
  match pretty_print_stream(&mut input, &mut output, indent) {
    Ok(_) => {},
    Err(f) => { return Err(f.to_string()); },
  };
  let output_string = match String::from_utf8(output) {
    Ok(s) => { s },
    Err(f) => { return Err(f.to_string()); },
  };
  return Ok(output_string);
}

/// Pretty-prints a stream of JSON-encoded data.
///
/// Valid inputs produce valid outputs.  However, this function does
/// *not* perform JSON validation, and is not guaranteed to either
/// reject or accept invalid input.
///
/// The `indent` parameter sets the string used to indent pretty-printed
/// output; e.g., `"  "` or `"\t"`.
///
/// `pretty_print_stream` uses `std::io::BufReader` and `std::io:BufWriter`
/// to provide IO buffering; no external buffering should be necessary.
///
/// # Example
///
/// ```no_run
/// match jsonxf::pretty_print_stream(&mut std::io::stdin(), &mut std::io::stdout(), "\t") {
///   Ok(_) => { },
///   Err(e) => { panic!(e.to_string()) }
/// };
/// ```
///
pub fn pretty_print_stream(input: &mut Read, output: &mut Write, indent: &str) -> Result<(), Error> {
  let mut reader = BufReader::new(input);
  let mut writer = BufWriter::new(output);
  let mut buf = [0 as u8; BUF_SIZE];
  let mut state = PpState {
    indent: indent,
    depth: 0,
    in_string: false,
    in_backslash: false,
    empty: false,
    wbuf: String::from(""),
  };
  loop {
    match reader.read(&mut buf) {
      Ok(0) => { break; },
      Ok(n) => { pp_buf(&buf[0..n], &mut writer, &mut state)?; },
      Err(e) => { return Err(e); },
    }
  }
  return Ok(());
}


/// Minimizes a string of JSON-encoded data.
///
/// Valid inputs produce valid outputs.  However, this function does
/// *not* perform JSON validation, and is not guaranteed to either
/// reject or accept invalid input.
///
/// # Examples:
///
/// ```
/// assert_eq!(
///   jsonxf::minimize("{ \"a\": \"b\", \"c\": 0 } ").unwrap(),
///   "{\"a\":\"b\",\"c\":0}"
/// );
/// assert_eq!(
///   jsonxf::minimize("\r\n\tnull\r\n").unwrap(),
///   "null"
/// );
/// ```
///
pub fn minimize(json_string: &str) -> Result<String, String> {
  let mut input = json_string.as_bytes();
  let mut output: Vec<u8> = vec![];
  match minimize_stream(&mut input, &mut output) {
    Ok(_) => {},
    Err(f) => { return Err(f.to_string()); },
  };
  let output_string = match String::from_utf8(output) {
    Ok(s) => { s },
    Err(f) => { return Err(f.to_string()); },
  };
  return Ok(output_string);
}


/// Minimizes a stream of JSON-encoded data.
///
/// Valid inputs produce valid outputs.  However, this function does
/// *not* perform JSON validation, and is not guaranteed to either
/// reject or accept invalid input.
///
/// `minimize_stream` uses `std::io::BufReader` and `std::io:BufWriter`
/// to provide IO buffering; no external buffering should be necessary.
///
/// # Example
///
/// ```no_run
/// match jsonxf::minimize_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
///   Ok(_) => { },
///   Err(e) => { panic!(e.to_string()) }
/// };
/// ```
///
pub fn minimize_stream(input: &mut Read, output: &mut Write) -> Result<(), Error> {
  let mut reader = BufReader::new(input);
  let mut writer = BufWriter::new(output);
  let mut buf = [0 as u8; BUF_SIZE];
  let mut state = MinState {
    in_string: false,
    in_backslash: false,
    depth: 0,
    print_newline: false,
  };
  loop {
    match reader.read(&mut buf) {
      Ok(0) => { break; },
      Ok(n) => { min_buf(&buf[0..n], &mut writer, &mut state)?; },
      Err(e) => { return Err(e); },
    }
  }
  return Ok(());
}



/* Pretty-prints the contents of `buf` into `writer`. */
fn pp_buf(buf: &[u8], writer: &mut Write, state: &mut PpState) -> Result<(),Error> {
  for n in 0..buf.len() {
    let b = buf[n];

    if state.in_string {
      writer.write(&buf[n..n+1])?;
      if state.in_backslash { state.in_backslash = false; }
      else if b == C_QUOTE { state.in_string = false; }
      else if b == C_BACKSLASH { state.in_backslash = true; }
    }
    else {
      match b {
        C_SPACE | C_LF | C_CR | C_TAB => {
          // skip whitespace
        },

        C_LEFT_BRACKET | C_LEFT_BRACE => {
          writer.write(&buf[n..n+1])?;
          state.depth += 1;
          // don't write trailing whitespace immediately, in case this
          // is an empty structure
          state.empty = true;
          state.wbuf = String::from("\n");
          for _ in 0..state.depth {
            state.wbuf.push_str(state.indent);
          }
        },

        C_RIGHT_BRACKET | C_RIGHT_BRACE => {
          state.depth -= 1;
          if state.empty {
            writer.write(&buf[n..n+1])?;
            state.empty = false;
          }
          else {
            writer.write(&['\n' as u8])?;
            for _ in 0..state.depth {
              writer.write(state.indent.as_bytes())?;
            }
            writer.write(&buf[n..n+1])?;
          }
          if state.depth == 0 {
            writer.write(&['\n' as u8])?;
          }
        },

        C_COMMA => {
          writer.write(&[b, '\n' as u8])?;
          for _ in 0..state.depth {
            writer.write(state.indent.as_bytes())?;
          }
        },

        C_COLON => {
          writer.write(&[':' as u8, ' ' as u8])?;
        },

        _ => {
          if state.empty {
            writer.write(state.wbuf.as_bytes())?;
            state.empty = false;
          }
          if b == C_QUOTE {
            state.in_string = true;
          }
          writer.write(&buf[n..n+1])?;
        },
      };
    };
  };

  return Ok(());
}


/* Minimizes the contents of `buf` into `writer`. */
fn min_buf(buf: &[u8], writer: &mut Write, state: &mut MinState) -> Result<(), Error> {
  for n in 0..buf.len() {
    let b = buf[n];

    if state.in_string {
      writer.write(&buf[n..n+1])?;
      if state.in_backslash { state.in_backslash = false; }
      else if b == C_QUOTE { state.in_string = false; }
      else if b == C_BACKSLASH { state.in_backslash = true; }
    }
    else {
      match b {
        C_SPACE | C_LF | C_CR | C_TAB => {
          // skip whitespace
        },

        C_LEFT_BRACKET | C_LEFT_BRACE => {
          if state.depth == 0 {
            if state.print_newline {
              writer.write(&[C_LF])?;
            }
            else {
              state.print_newline = true;
            }
          }
          state.depth += 1;
          writer.write(&buf[n..n+1])?;
        },

        C_RIGHT_BRACKET | C_RIGHT_BRACE => {
          state.depth -= 1;
          writer.write(&buf[n..n+1])?;
        },

        _ => {
          if b == C_QUOTE {
            state.in_string = true;
          }
          writer.write(&buf[n..n+1])?;
        },
      };
    };
  };

  return Ok(());
}




