use clap::Clap;
use parse::program::Program;
use interpret::interpreter::Interpreter;

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
