use std::vec::Vec;

use parse::program::Program;

use crate::util::{ChooserDirection, PointerDirection};

/// An interpreter for a Piet program.
///
/// # Parameters
///
/// * `pointer_x` - The x coordinate of the direction pointer.
/// * `pointer_y` - The y coordinate of the direction pointer.
/// * `pointer_direction` - The direction of the direction pointer.
/// * `chooser_direction` - The direction of the direction chooser.
/// * `stack` - The stack for storing data values.
/// * `program` - The program being run by the interpreter.
pub struct Interpreter {
    pointer_x: u32,
    pointer_y: u32,
    pointer_direction: PointerDirection,
    chooser_direction: ChooserDirection,
    stack: Vec<u32>,
    program: Program,
}

impl Interpreter {
    /// Initializes the interpreter with a program.
    pub fn new(program: Program) -> Self {
        Interpreter {
            pointer_x: 0,
            pointer_y: 0,
            pointer_direction: PointerDirection::Right,
            chooser_direction: ChooserDirection::Left,
            stack: Vec::new(),
            program,
        }
    }

    /// Runs the interpreter until completion.
    pub fn run(&mut self) -> () {}
}
