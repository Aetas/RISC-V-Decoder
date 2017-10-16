extern crate getopts;
use getopts::Options;   //argument options
use std::env;           //environment variables


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();


    println!("Hello, world!");
}
