use clap::Clap;

use interpret::interpreter::Interpreter;
use parse::program::Program;

#[derive(Clap)]
struct Opts {
    file: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let program = Program::load(&opts.file);
    let mut interpreter = Interpreter::new(program);
    interpreter.run();
}
