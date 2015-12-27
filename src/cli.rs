use getopts::Options;
use std::env;

use template;
use template::Data;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "config", "configuration file to use", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("[error]: {}", f);
            return print_usage(&program, opts);
        }
    };
    if matches.opt_present("h") {
        return print_usage(&program, opts);
    }

    let mut data = Data::new();
    data.insert(String::from("var"), String::from("world"));

    println!("{}", template::load("./src/test/template.hbs").ok().unwrap().render(&data));

    return;
}
