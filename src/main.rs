use clap::Clap;

use interpret::interpreter::Interpreter;
use parse::program::Program;

use std::io::{self, Read};

#[derive(Clap)]
struct Opts {
    file: String,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let stdin = buffer.chars().collect();

    let opts: Opts = Opts::parse();
    let program = Program::load(&opts.file);
    let mut interpreter = Interpreter::new(program, stdin);
    interpreter.run();
}
