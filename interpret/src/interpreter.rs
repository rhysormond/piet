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
    pub fn new(program: Program, stdin: Vec<char>) -> Self {
        Interpreter {
            program,
            state: State::new(stdin),
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
                let current_codel = &self.program.codel_at(self.state.pointer_location);
                // If we passed through or are in a white region then we never execute a command
                if current_codel == &&Codel::White || passed_white {
                    (0, 0)
                } else {
                    // TODO: The codels should both always be colors, tighten this guarantee
                    current_codel.compare(&next_codel).unwrap()
                }
            };
            let current_region_size = self.program.region_at(self.state.pointer_location).size;
            execute(
                &mut self.state,
                delta_hue,
                delta_lightness,
                current_region_size,
            );
            self.state.pointer_location = next_location;
            // Reset the termination counter if we've found a path forwards
            self.state.termination_counter = 0;
        } else {
            // For even attempts, advance the chooser direction; otherwise alternate the pointer direction
            if self.state.termination_counter % 2 == 0 {
                self.state.chooser_direction = self.state.chooser_direction.next();
            } else {
                self.state.pointer_direction = self.state.pointer_direction.next();
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
        let first_edge = self.disjoint_edge_coordinate(self.state.pointer_location.clone(), self.state.pointer_direction);
        let second_edge = self.disjoint_edge_coordinate(first_edge, self.state.pointer_direction.with_chooser(self.state.chooser_direction));

        // Check if we're moving into:
        //  - either the edge of the program or a black codel in which case we stop
        //  - a white codel in which case we:
        //    - find and move to the first edge and then stop (even if there are other non-contiguous ones later)
        //    - step into it if it's a colored codel, otherwise stay in the current white codel
        //  - a colored codel in which case we step one square into it and stop
        if let Some((next_location, next_codel)) = self.program.maybe_next_codel(second_edge, self.state.pointer_direction) {
            match next_codel {
                Codel::Black => None,
                Codel::White => {
                    // Find the edge of the white region
                    let white_edge = self.edge_coordinate(next_location, self.state.pointer_direction);
                    // If we're about to step into a colored codel, do it; otherwise, stop at the edge
                    match self.program.maybe_next_codel(white_edge, self.state.pointer_direction) {
                        // TODO: ideally this guard would be written to just ensure that this IS colored rather than NOT everything else
                        Some((color_location, color_codel)) if color_codel != &Codel::White && color_codel != &Codel::Black => {
                            Some((color_location, color_codel, true))
                        }
                        _ => Some((white_edge, &Codel::White, true)),
                    }
                }
                _ => Some((next_location, next_codel, false)),
            }
        } else {
            None
        }
    }

    /// The coordinate of the closest region edge (exclusive) reached starting from `start` and moving in `direction`.
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

    /// The coordinate of the farthest region edge (exclusive) reached starting from `start` and moving in `direction`.
    /// TODO: this needs to support disjoint regions, we can't just assume the first edge that we hit is the last one
    fn disjoint_edge_coordinate(&self, start: (usize, usize), direction: PointerDirection) -> (usize, usize) {
        self.edge_coordinate(start, direction)
    }
}
