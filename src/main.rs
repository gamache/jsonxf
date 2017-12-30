/*
  Jsonxf: a fast JSON pretty-printer and minimizer.
   ,
  (k) 2017-2018 pete gamache <pete@gamache.org>

  Input must be in UTF-8 encoding.
  Jsonxf does not hold strong opinions on JSON validity.
  It is designed for speed.

  Run `jsonxf -h` for usage options.
*/

use std::fs::File;

extern crate jsonxf;
use jsonxf::pretty_print_stream;
use jsonxf::minimize_stream;

extern crate getopts;
use getopts::Options;


fn main() {
  match do_main() {
    Ok(_) => { },
    Err(e) => {
      eprintln!("{}", e);
      std::process::exit(1);
    }
  };
}

fn do_main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().collect();

  let mut opts = Options::new();
  opts.optopt("i", "input", "read input from the given file (default: STDIN)", "file");
  opts.optopt("o", "output", "write output to the given file (default: STDOUT)", "file");
  opts.optopt("t", "tab", "use the given string to indent pretty-printed output (default: two spaces)", "tabstr");
  opts.optflag("m", "minimize", "minimize JSON instead of pretty-printing it");
  opts.optflag("h", "help", "print this message and exit");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(e) => {
      return Err(e.to_string());
    }
  };

  if matches.opt_present("h") {
    let program = args[0].clone();
    print_help(&program, &opts);
    return Ok(());
  }

  let mut input: Box<std::io::Read> = match matches.opt_str("i") {
    None => {
      Box::new(std::io::stdin())
    },
    Some(filename) => {
      if filename == "-".to_owned() {
        Box::new(std::io::stdin())
      }
      else {
        match File::open(&filename) {
          Ok(f) => {
            Box::new(f)
          },
          Err(e) => {
            let mut estr = String::from(filename);
            estr.push_str(": ");
            estr.push_str(&e.to_string());
            return Err(estr);
          }
        }
      }
    },
  };

  let mut output: Box<std::io::Write> = match matches.opt_str("o") {
    None => {
      Box::new(std::io::stdout())
    },
    Some(filename) => {
      if filename == "-".to_owned() {
        Box::new(std::io::stdout())
      }
      else {
        match File::create(&filename) {
          Ok(f) => {
            Box::new(f)
          },
          Err(e) => {
            let mut estr = String::from(filename);
            estr.push_str(": ");
            estr.push_str(&e.to_string());
            return Err(estr);
          }
        }
      }
    },
  };

  let indent: String = match matches.opt_str("t") {
    None => { String::from("  ") },
    Some(string) => { string.clone() },
  };

  let result = if matches.opt_present("m") {
    minimize_stream(&mut input, &mut output)
  }
  else {
    pretty_print_stream(&mut input, &mut output, &indent)
  };

  match result {
    Err(e) => { Err(e.to_string()) },
    Ok(_) => Ok(())
  }
}

fn print_help(program_name: &str, opts: &Options) -> () {
  let desc =
"Jsonxf is a JSON transformer.  It provides fast pretty-printing and
minimizing of JSON-encoded UTF-8 data.

Pretty-print example:

    jsonxf <foo.json >foo-pretty.json

Minimize example:

    jsonxf -m <foo.json >foo-min.json";

  let brief = format!("Usage: {} [options]\n\n{}", program_name, desc);
  print!("{}", opts.usage(&brief));
}

