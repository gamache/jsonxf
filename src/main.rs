/*
  Jsonxf: a fast JSON pretty-printer and minimizer.
   ,
  (k) 2017-2018 pete gamache <pete@gamache.org>

  Input must be in UTF-8 encoding.
  Jsonxf does not hold strong opinions on JSON validity.
  It is designed for speed.
*/

use std::fs::File;

extern crate jsonxf;
use jsonxf::pretty_print_stream;
use jsonxf::minimize_stream;

extern crate getopts;
use getopts::Options;


fn main() {
  let args: Vec<String> = std::env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("i", "input", "read input from the given file (default: STDIN)", "file");
  opts.optopt("o", "output", "write output to the given file (default: STDOUT)", "file");
  opts.optopt("t", "tab", "use the given string to indent pretty-printed output (default: two spaces)", "tabstr");
  opts.optflag("m", "minimize", "minimize JSON instead of pretty-printing it");
  opts.optflag("h", "help", "print this message");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_help(&program, &opts);
    return;
  }

  let mut input: Box<std::io::Read> = match matches.opt_str("i") {
    None => { Box::new(std::io::stdin()) },
    Some(filename) => {
      if filename == "-".to_owned() {
        Box::new(std::io::stdin())
      }
      else {
        match File::open(filename) {
          Ok(file) => { Box::new(file) },
          Err(f) => { panic!(f.to_string()) }
        }
      }
    },
  };

  let mut output: Box<std::io::Write> = match matches.opt_str("o") {
    None => { Box::new(std::io::stdout()) },
    Some(filename) => {
      if filename == "-".to_owned() {
        Box::new(std::io::stdout())
      }
      else {
        match File::create(filename) {
          Ok(file) => { Box::new(file) },
          Err(f) => { panic!(f.to_string()) }
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
    Ok(_) => (),
    Err(e) => println!("Error: {}", e),
  }
}

fn print_help(program_name: &str, opts: &Options) -> () {
  let brief = format!("Usage: {} [options]", program_name);
  print!("{}", opts.usage(&brief));
}

