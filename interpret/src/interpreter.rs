use parse::program::Program;

use crate::command::execute;
use crate::state::State;

/// An interpreter for a Piet program.
///
/// # Parameters
///
/// * `program` - The program being run by the interpreter.
/// * `state` - The state of the interpreter.
pub struct Interpreter {
    program: Program,
    state: State,
}

impl Interpreter {
    /// Initializes the interpreter with a program.
    pub fn new(program: Program) -> Self {
        Interpreter {
            program,
            state: State::new(),
        }
    }

    /// Runs the interpreter until completion.
    pub fn run(&mut self) -> () {
        self.advance();
    }

    /// Advance the program state by one iteration.
    pub fn advance(&mut self) -> () {
        let hue_change = todo!();
        let lightness_change = todo!();
        let last_region_size = todo!();
        execute(&mut self.state, hue_change, lightness_change, last_region_size);
    }
}
