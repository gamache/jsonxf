//! Jsonxf is a JSON transformer, supporting fast pretty-printing,
//! minimizing, and similar formatting of JSON-encoded UTF-8 data
//! in string or stream format.  Use it to customize the
//! presentation of valid JSON data.
//!
//! It provides default pretty-printers (`pretty_print()`,
//! `pretty_print_stream()`) and minimizers (`minimize()`,
//! `minimize_stream()`), `Formatter` for customizable JSON
//! formatting, and the `jsonxf` command-line tool (see
//! `jsonxf -h` for options).
//!
//! Jsonxf is built for speed, and does not attempt to perform any
//! input validation whatsoever.  Valid input produces valid output,
//! but no guarantees are offered around the detection and rejection
//! of invalid input.
//!
//! GitHub:
//! <a href="https://github.com/gamache/jsonxf" target="_blank">gamache/jsonxf</a>
//!

use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;

const BUF_SIZE: usize = 1024 * 16;

const C_CR: u8 = '\r' as u8;
const C_LF: u8 = '\n' as u8;
const C_TAB: u8 = '\t' as u8;
const C_SPACE: u8 = ' ' as u8;

const C_COMMA: u8 = ',' as u8;
const C_COLON: u8 = ':' as u8;
const C_QUOTE: u8 = '"' as u8;
const C_BACKSLASH: u8 = '\\' as u8;

const C_LEFT_BRACE: u8 = '{' as u8;
const C_LEFT_BRACKET: u8 = '[' as u8;
const C_RIGHT_BRACE: u8 = '}' as u8;
const C_RIGHT_BRACKET: u8 = ']' as u8;

/// `Formatter` allows customizable pretty-printing, minimizing,
/// and other formatting tasks on JSON-encoded UTF-8 data in
/// string or stream format.
///
/// Example:
///
/// ```
/// let mut fmt = jsonxf::Formatter::pretty_printer();
/// fmt.line_separator = String::from("\r\n");
/// assert_eq!(
///     fmt.format("{\"a\":1}").unwrap(),
///     "{\r\n  \"a\": 1\r\n}"
/// );
/// ```
pub struct Formatter {
    /// Used for beginning-of-line indentation in arrays and objects.
    pub indent: String,

    /// Used inside arrays and objects.
    pub line_separator: String,

    /// Used between root-level arrays and objects.
    pub record_separator: String,

    /// Used after a colon inside objects.
    pub after_colon: String,

    /// Used at very end of output.
    pub trailing_output: String,

    // private mutable state
    depth: usize, // current nesting depth
    in_string: bool, // is the next byte part of a string?
    in_backslash: bool, // does the next byte follow a backslash in a string?
    empty: bool, // is the next byte in an empty object or array?
    first: bool, // is this the first byte of input?
}

impl Formatter {
    fn default() -> Formatter {
        Formatter {
            indent: String::from("  "),
            line_separator: String::from("\n"),
            record_separator: String::from("\n"),
            after_colon: String::from(" "),
            trailing_output: String::from(""),
            depth: 0,
            in_string: false,
            in_backslash: false,
            empty: false,
            first: true,
        }
    }

    /// Returns a Formatter set up for pretty-printing.
    /// Defaults to using two spaces of indentation,
    /// Unix newlines, and no whitespace at EOF.
    ///
    /// # Example:
    ///
    /// ```
    /// assert_eq!(
    ///     jsonxf::Formatter::pretty_printer().format("{\"a\":1}").unwrap(),
    ///     "{\n  \"a\": 1\n}"
    /// );
    /// ```
    pub fn pretty_printer() -> Formatter {
        return Formatter::default();
    }

    /// Returns a Formatter set up for minimizing.
    /// Defaults to using Unix newlines between records,
    /// and no whitespace at EOF.
    ///
    /// # Example:
    ///
    /// ```
    /// assert_eq!(
    ///     jsonxf::Formatter::minimizer().format("{  \"a\" : 1  }\n").unwrap(),
    ///     "{\"a\":1}"
    /// );
    /// ```
    pub fn minimizer() -> Formatter {
        let mut xf = Formatter::default();
        xf.indent = String::from("");
        xf.line_separator = String::from("");
        xf.record_separator = String::from("\n");
        xf.after_colon = String::from("");
        return xf;
    }

    /// Formats a string of JSON-encoded data.
    ///
    /// Input must be valid JSON data in UTF-8 encoding.
    ///
    /// # Example:
    ///
    /// ```
    /// let mut fmt = jsonxf::Formatter::pretty_printer();
    /// fmt.indent = String::from("\t");
    /// fmt.trailing_output = String::from("\n");
    /// assert_eq!(
    ///     fmt.format("{\"a\":1}").unwrap(),
    ///     "{\n\t\"a\": 1\n}\n"
    /// );
    /// ```
    pub fn format(&mut self, json_string: &str) -> Result<String, String> {
        let mut input = json_string.as_bytes();
        let mut output: Vec<u8> = vec![];
        match self.format_stream(&mut input, &mut output) {
            Ok(_) => {}
            Err(f) => {
                return Err(f.to_string());
            }
        };
        let output_string = match String::from_utf8(output) {
            Ok(s) => s,
            Err(f) => {
                return Err(f.to_string());
            }
        };
        return Ok(output_string);
    }

    /// Formats a stream of JSON-encoded data.
    ///
    /// Input must be valid JSON data in UTF-8 encoding.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// let mut fmt = jsonxf::Formatter::pretty_printer();
    /// fmt.indent = String::from("\t");
    /// fmt.trailing_output = String::from("\n");
    /// match fmt.format_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
    ///     Ok(_) => { /* YAY */ },
    ///     Err(e) => { panic!(e.to_string()); }
    /// }
    /// ```
    pub fn format_stream(&mut self, input: &mut Read, output: &mut Write) -> Result<(), Error> {
        let mut reader = BufReader::new(input);
        let mut writer = BufWriter::new(output);
        let mut buf = [0 as u8; BUF_SIZE];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    self.format_buf(&buf[0..n], &mut writer)?;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        writer.write(self.trailing_output.as_bytes())?;
        return Ok(());
    }


    /* Formats the contents of `buf` into `writer`. */
    fn format_buf(&mut self, buf: &[u8], writer: &mut Write) -> Result<(), Error> {
        for n in 0..buf.len() {
            let b = buf[n];

            if self.in_string {
                writer.write(&buf[n..n + 1])?;
                if self.in_backslash {
                    self.in_backslash = false;
                } else if b == C_QUOTE {
                    self.in_string = false;
                } else if b == C_BACKSLASH {
                    self.in_backslash = true;
                }
            } else {
                match b {
                    C_SPACE | C_LF | C_CR | C_TAB => {
                        // skip whitespace
                    }

                    C_LEFT_BRACKET | C_LEFT_BRACE => {
                        if self.first {
                            self.first = false;
                            writer.write(&buf[n..n + 1])?;
                        } else if self.empty {
                            writer.write(self.line_separator.as_bytes())?;
                            for _ in 0..self.depth {
                                writer.write(self.indent.as_bytes())?;
                            }
                            writer.write(&buf[n..n + 1])?;

                        } else if self.depth == 0 {
                            writer.write(self.record_separator.as_bytes())?;
                            writer.write(&buf[n..n + 1])?;
                        } else {
                            writer.write(&buf[n..n + 1])?;
                        }
                        self.depth += 1;
                        self.empty = true;
                    }

                    C_RIGHT_BRACKET | C_RIGHT_BRACE => {
                        self.depth -= 1;
                        if self.empty {
                            self.empty = false;
                            writer.write(&buf[n..n + 1])?;
                        } else {
                            writer.write(self.line_separator.as_bytes())?;
                            for _ in 0..self.depth {
                                writer.write(self.indent.as_bytes())?;
                            }
                            writer.write(&buf[n..n + 1])?;
                        }
                    }

                    C_COMMA => {
                        writer.write(&buf[n..n + 1])?;
                        writer.write(self.line_separator.as_bytes())?;
                        for _ in 0..self.depth {
                            writer.write(self.indent.as_bytes())?;
                        }
                    }

                    C_COLON => {
                        writer.write(&buf[n..n + 1])?;
                        writer.write(self.after_colon.as_bytes())?;
                    }

                    _ => {
                        if self.empty {
                            writer.write(self.line_separator.as_bytes())?;
                            for _ in 0..self.depth {
                                writer.write(self.indent.as_bytes())?;
                            }
                            self.empty = false;
                        }
                        if b == C_QUOTE {
                            self.in_string = true;
                        }
                        writer.write(&buf[n..n + 1])?;
                    }
                };
            };
        }

        return Ok(());
    }
}

/// Pretty-prints a string of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use two spaces as an indent, a line feed
/// as newline character, and no trailing whitespace.
/// To customize this behavior, use a
/// `jsonxf::Formatter::pretty_printer()` directly.
///
/// # Examples:
///
/// ```
/// assert_eq!(
///     jsonxf::pretty_print("{\"a\":1,\"b\":2}").unwrap(),
///     "{\n  \"a\": 1,\n  \"b\": 2\n}"
/// );
/// assert_eq!(
///     jsonxf::pretty_print("{\"empty\":{},\n\n\n\n\n\"one\":[1]}").unwrap(),
///     "{\n  \"empty\": {},\n  \"one\": [\n    1\n  ]\n}"
/// );
/// ```
///
pub fn pretty_print(json_string: &str) -> Result<String, String> {
    Formatter::pretty_printer().format(json_string)
}

/// Pretty-prints a stream of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use two spaces as an indent, a line feed
/// as newline character, and no trailing whitespace.
/// To customize this behavior, use a
/// `jsonxf::Formatter::pretty_printer()` directly.
///
/// `pretty_print_stream` uses `std::io::BufReader` and `std::io:BufWriter`
/// to provide IO buffering; no external buffering should be necessary.
///
/// # Example:
///
/// ```no_run
/// match jsonxf::pretty_print_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
///     Ok(_) => { /* YAY */ },
///     Err(e) => { panic!(e.to_string()) }
/// };
/// ```
///
pub fn pretty_print_stream(input: &mut Read, output: &mut Write) -> Result<(), Error> {
    Formatter::pretty_printer().format_stream(input, output)
}

/// Minimizes a string of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use a line feed as newline character between
/// records, and no trailing whitespace.  To customize this behavior,
/// use a `jsonxf::Formatter::minimizer()` directly.
///
/// # Examples:
///
/// ```
/// assert_eq!(
///     jsonxf::minimize("{ \"a\": \"b\", \"c\": 0 } ").unwrap(),
///     "{\"a\":\"b\",\"c\":0}"
/// );
/// assert_eq!(
///     jsonxf::minimize("\r\n\tnull\r\n").unwrap(),
///     "null"
/// );
/// ```
///
pub fn minimize(json_string: &str) -> Result<String, String> {
    Formatter::minimizer().format(json_string)
}

/// Minimizes a stream of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use a line feed as newline character between
/// records, and no trailing whitespace.  To customize this behavior,
/// use a `jsonxf::Formatter::minimizer()` directly.
///
/// `minimize_stream` uses `std::io::BufReader` and `std::io:BufWriter`
/// to provide IO buffering; no external buffering should be necessary.
///
/// # Example:
///
/// ```no_run
/// match jsonxf::minimize_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
///     Ok(_) => { /* YAY */ },
///     Err(e) => { panic!(e.to_string()) }
/// };
/// ```
///
pub fn minimize_stream(input: &mut Read, output: &mut Write) -> Result<(), Error> {
    Formatter::minimizer().format_stream(input, output)
}
