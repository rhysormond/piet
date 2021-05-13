use crate::direction::{PointerDirection, ChooserDirection};

/// The state of a Piet program.
///
/// # Parameters
///
/// * `pointer_x` - The x coordinate of the direction pointer.
/// * `pointer_y` - The y coordinate of the direction pointer.
/// * `pointer_direction` - The direction of the direction pointer.
/// * `chooser_direction` - The direction of the direction chooser.
/// * `stack` - The stack for storing data values.
/// * `termination_counter` - The number of times that the program has failed to advance.
pub struct State {
    pub(crate) pointer_x: usize,
    pub(crate) pointer_y: usize,
    pub(crate) pointer_direction: PointerDirection,
    pub(crate) chooser_direction: ChooserDirection,
    pub(crate) stack: Vec<isize>,
    pub(crate) termination_counter: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            pointer_x: 0,
            pointer_y: 0,
            pointer_direction: PointerDirection::Right,
            chooser_direction: ChooserDirection::Left,
            stack: Vec::new(),
            termination_counter: 0,
        }
    }
}
