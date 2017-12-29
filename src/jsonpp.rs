use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::fs::File;


pub fn pp(input: &mut Read, output: &mut Write, indent: &str) -> Result<(), Error> {
  let reader = BufReader::new(input);
  let mut writer = BufWriter::new(output);

  let mut in_string = false;
  let mut in_backslash = false;
  let mut depth = 0;
  let mut whitespace_buffer = String::from("");
  let mut current_structure_is_empty = false;

  for byte in reader.bytes() {
    let b = byte?;
    let c = b as char;

    if in_string {
      writer.write(&[b])?;
      if in_backslash { in_backslash = false; }
      else if c == '"' { in_string = false; }
      else if c == '\\' { in_backslash = true; }
    }
    else {
      match c {
        ' ' | '\t' | '\n' | '\r' => {
          /* skip whitespace */
         },

        '{' | '[' => {
          writer.write(&[b])?;
          depth += 1;
          // don't write trailing whitespace immediately, in case this
          // is an empty structure
          current_structure_is_empty = true;
          whitespace_buffer = String::from("\n");
          for _ in 0..depth {
            whitespace_buffer.push_str(indent);
          }
        },

        '}' | ']' => {
          depth -= 1;
          if current_structure_is_empty {
            writer.write(&[b])?;
            current_structure_is_empty = false;
          }
          else {
            writer.write(&['\n' as u8])?;
            for _ in 0..depth {
              writer.write(indent.as_bytes())?;
            }
            writer.write(&[b])?;
          }
          if depth == 0 {
            writer.write(&['\n' as u8])?;
          }
        },

        ',' => {
          writer.write(&[b, '\n' as u8])?;
          for _ in 0..depth {
            writer.write(indent.as_bytes())?;
          }
        },

        ':' => {
          writer.write(&[':' as u8, ' ' as u8])?;
        },

        c => {
          if current_structure_is_empty {
            writer.write(whitespace_buffer.as_bytes())?;
            current_structure_is_empty = false;
          }
          if c == '"' {
            in_string = true;
          }
          writer.write(&[b])?;
        },
      }
    }
  }
  return Ok(());
}

pub fn minimize(input: &mut Read, output: &mut Write) -> Result<(), Error> {
  let reader = BufReader::new(input);
  let mut writer = BufWriter::new(output);

  let mut in_string = false;
  let mut in_backslash = false;
  let mut depth = 0;

  for byte in reader.bytes() {
    let b = byte?;
    let c = b as char;

    if in_string {
      writer.write(&[b])?;
      if in_backslash { in_backslash = false; }
      else if c == '"' { in_string = false; }
      else if c == '\\' { in_backslash = true; }
    }
    else {
      match c {
        ' ' | '\t' | '\n' | '\r' => {
          /* skip whitespace */
         },

        '{' | '[' => {
          writer.write(&[b])?;
          depth += 1;
        },

        '}' | ']' => {
          writer.write(&[b])?;
          depth -= 1;
          if depth == 0 {
            writer.write(&['\n' as u8])?;
          }
        },

        c => {
          if c == '"' {
            in_string = true;
          }
          writer.write(&[b])?;
        },
      }
    }
  }
  return Ok(());
}

