use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::fs::File;

extern crate jsonpp;
use jsonpp::pp;
use jsonpp::minimize;

extern crate getopts;
use getopts::Options;


/*
  A JSON pretty-printer and minimizer.  Pretty fast.
  Does not hold strong opinions on JSON validity.
  Input should be ASCII or UTF-8; other encodings are not guaranteed to work.
*/

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("i", "input", "read input from the given file (default: STDIN)", "file");
  opts.optopt("o", "output", "write output to the given file (default: STDOUT)", "file");
  opts.optopt("t", "tab", "use the given string to indent pretty-printed output (default: two spaces)", "tabstr");
  opts.optflag("m", "minimize", "minimize JSON instead of pretty-printing it");
  opts.optflag("h", "help", "print this help message");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_help(&program, &opts);
    return;
  }

  let mut input: Box<io::Read> = match matches.opt_str("i") {
    None => { Box::new(io::stdin()) },
    Some(filename) => {
      if filename == "-".to_owned() {
        Box::new(io::stdin())
      }
      else {
        match File::open(filename) {
          Ok(file) => { Box::new(file) },
          Err(f) => { panic!(f.to_string()) }
        }
      }
    },
  };

  let mut output: Box<io::Write> = match matches.opt_str("o") {
    None => { Box::new(io::stdout()) },
    Some(filename) => {
      if filename == "-".to_owned() {
        Box::new(io::stdout())
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
    minimize(&mut input, &mut output)
  }
  else {
    pp(&mut input, &mut output, &indent)
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

