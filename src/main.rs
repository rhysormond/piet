use clap::Clap;

use interpret::interpreter::Interpreter;
use parse::program::Program;

#[derive(Clap)]
struct Opts {
    file: String,
}

fn main() {
    let mut buffer = String::new();

    println!("reading a single line from stdin");
    std::io::stdin().read_line(&mut buffer).unwrap();

    let opts: Opts = Opts::parse();
    let program = Program::load(&opts.file);
    let mut interpreter = Interpreter::new(program, buffer.chars().collect());
    interpreter.run();
}
