use crate::chooser::Chooser;
use parse::direction::Direction;

/// The state of a Piet program.
///
/// # Parameters
///
/// * `pointer` - The (row, column) coordinates of the direction pointer (relative to the top left corner).
/// * `direction` - The primary direction of the pointer.
/// * `chooser` - The secondary direction of the pointer.
/// * `stack` - The stack for storing data values.
/// * `termination_counter` - The number of times that the program has failed to advance.
/// * `stdin` - Any stdin piped to the program.
#[derive(Debug)]
pub struct State {
    pub(crate) pointer: (usize, usize),
    pub(crate) direction: Direction,
    pub(crate) chooser: Chooser,
    pub(crate) stack: Vec<isize>,
    pub(crate) termination_counter: u8,
    pub(crate) stdin: Vec<char>,
}

impl State {
    /// NOTE: Reverses `stdin` as we want to simulate pushing each element onto the stack.
    pub fn new(stdin: Vec<char>) -> Self {
        Self {
            pointer: (0, 0),
            direction: Direction::Right,
            chooser: Chooser::Left,
            stack: Vec::new(),
            termination_counter: 0,
            stdin: stdin.into_iter().rev().collect(),
        }
    }
}
