use crate::direction::{ChooserDirection, PointerDirection};

/// The state of a Piet program.
///
/// # Parameters
///
/// * `pointer_location` - The (row, column) coordinates of the direction pointer (relative to the top left corner).
/// * `pointer_direction` - The direction of the direction pointer.
/// * `chooser_direction` - The direction of the direction chooser.
/// * `stack` - The stack for storing data values.
/// * `termination_counter` - The number of times that the program has failed to advance.
#[derive(Debug)]
pub struct State {
    pub(crate) pointer_location: (usize, usize),
    pub(crate) pointer_direction: PointerDirection,
    pub(crate) chooser_direction: ChooserDirection,
    pub(crate) stack: Vec<isize>,
    pub(crate) termination_counter: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            pointer_location: (0, 0),
            pointer_direction: PointerDirection::Right,
            chooser_direction: ChooserDirection::Left,
            stack: Vec::new(),
            termination_counter: 0,
        }
    }
}
