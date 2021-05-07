use clap::Clap;
use parse::program::Program;

#[derive(Clap)]
struct Opts {
    file: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    Program::load(&opts.file);
}
