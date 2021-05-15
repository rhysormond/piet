use parse::codel::Codel;
use parse::program::Program;

use crate::command::execute;
use crate::direction::PointerDirection;
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
        while self.state.termination_counter < 8 {
            self.advance();
        }
    }

    /// Advance the program state by one iteration.
    fn advance(&mut self) -> () {
        if let Some((next_location, next_codel, passed_white)) = self.next_coordinates() {
            // If this is a region that we can move into, do it!
            let (delta_hue, delta_lightness) = {
                // If we passed through a white region then we never execute a command
                if passed_white {
                    (0, 0)
                } else {
                    let (row, col) = self.state.pointer_location;
                    let current_codel = &self.program.codels[row][col];
                    // TODO: The codels should both always be colors, tighten this guarantee
                    current_codel.compare(&next_codel).unwrap()
                }
            };
            let current_region_size = *self.program.regions.get(&self.state.pointer_location).unwrap();
            execute(
                &mut self.state,
                delta_hue,
                delta_lightness,
                current_region_size,
            );
            self.state.pointer_location = next_location;
        } else {
            // For even attempts, advance the pointer direction; otherwise alternate the chooser direction
            if self.state.termination_counter % 2 == 0 {
                self.state.pointer_direction = self.state.pointer_direction.next();
            } else {
                self.state.chooser_direction = self.state.chooser_direction.next();
            }
            self.state.termination_counter += 1;
        }
    }

    /// Returns the next coordinates of the display pointer
    ///
    /// # Returns
    /// If the next region exists:
    ///  - coordinates of the next region
    ///  - the next region's color
    ///  - whether a white region was traversed
    /// TODO: the return type can be strengthened to Codel::Color without too much work
    fn next_coordinates(&self) -> Option<((usize, usize), &Codel, bool)> {
        let first_edge = self.edge_coordinate(self.state.pointer_location.clone(), self.state.pointer_direction);
        let second_edge = self.edge_coordinate(first_edge, self.state.pointer_direction.with_chooser(self.state.chooser_direction));

        // Check if we're moving into a new codel or just off the edge of the program
        // TODO: this could be trivially turned into a .map
        if let Some((next_location, next_codel)) = self.program.maybe_next_codel(second_edge, self.state.pointer_direction) {
            match next_codel {
                Codel::Black => None,
                Codel::White => {
                    // Find the first non-white codel in the same direction
                    let white_edge = self.edge_coordinate(next_location, self.state.pointer_direction);
                    self
                        .program
                        .maybe_next_codel(white_edge, self.state.pointer_direction)
                        .map(|(point, codel)| (point, codel, true))
                }
                _ => Some((next_location, next_codel, false)),
            }
        } else {
            None
        }
    }

    /// The coordinate of the first region edge (exclusive) reached starting from `start` and moving in `direction`.
    fn edge_coordinate(&self, start: (usize, usize), direction: PointerDirection) -> (usize, usize) {
        let codel = self.program.codel_at(start);
        let mut pointer = start;
        let mut edge = false;
        while edge == false {
            match self.program.maybe_next_codel(pointer, direction) {
                Some((next_pointer, next_codel)) if next_codel == codel => {
                    pointer = next_pointer
                }
                _ => edge = true
            }
        };
        pointer
    }
}
