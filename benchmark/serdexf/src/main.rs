extern crate serde_json;
extern crate serde_transcode;

use std::io::{self, Read, Write};

// This is lightly-altered sample code from the serde_transcode project,
// and is included in order to compare the speeds of Jsonxf (a library
// which does almost nothing) and the serde libraries (whose scope and
// power is formidable).

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let stdin = io::stdin();
  let stdout = io::stdout();

  let mut input = String::new();
  stdin.lock().read_to_string(&mut input).unwrap();
  let mut output = Vec::with_capacity(input.len());

  {
    let mut d = serde_json::Deserializer::from_str(&input);
    if args.len() > 1 && args[1] == "-m" {
      let mut s = serde_json::Serializer::new(&mut output);
      while serde_transcode::transcode(&mut d, &mut s).is_ok() {}
    }
    else {
      let mut s = serde_json::Serializer::pretty(&mut output);
      while serde_transcode::transcode(&mut d, &mut s).is_ok() {}
    };
  }

  stdout.lock().write_all(&output).unwrap();
}

