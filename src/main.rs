use ansi_term::Colour::{Red, Yellow};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", Red.paint("Expected arguments COLOR and <MESSAGE> passed!"));
        return
    }
    println!("{}", Yellow.paint(format!("The color you passed: {}", args[1])));
}
