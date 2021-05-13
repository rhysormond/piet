use std::vec::Vec;

use parse::program::Program;

use crate::state::State;
use crate::direction::{ChooserDirection, PointerDirection};

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
    pub fn run(&mut self) -> () {}
}
