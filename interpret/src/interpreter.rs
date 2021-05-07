use std::vec::Vec;

use parse::program::Program;

use crate::util::{ChooserDirection, PointerDirection};

pub struct Interpreter {
    pointer_x: u32,
    pointer_y: u32,
    pointer_direction: PointerDirection,
    chooser_direction: ChooserDirection,
    stack: Vec<u32>,
    program: Program,
}

impl Interpreter {
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

    pub fn run(&mut self) -> () {

    }
}
