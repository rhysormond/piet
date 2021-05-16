use parse::color::Color;
use parse::program::Program;

use crate::command::execute;
use crate::state::State;
use parse::direction::Direction;

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
    pub fn run(&mut self) {
        while self.state.termination_counter < 8 {
            self.advance();
        }
    }

    /// Advance the program state by one iteration.
    fn advance(&mut self) {
        if let Some((next_location, next_color, passed_white)) = self.next_coordinates() {
            // If this is a region that we can move into, do it!
            let (delta_hue, delta_lightness) = {
                let current_color = &self.program.color_at(self.state.pointer);
                // If we passed through or are in a white region then we never execute a command
                if current_color == &&Color::White || passed_white {
                    (0, 0)
                } else {
                    // TODO: The colors should both always be Color::Color, tighten this guarantee
                    current_color.compare(&next_color).unwrap()
                }
            };
            let current_region_size = self.program.region_at(self.state.pointer).size;
            execute(
                &mut self.state,
                delta_hue,
                delta_lightness,
                current_region_size,
            );
            self.state.pointer = next_location;
            // Reset the termination counter if we've found a path forwards
            self.state.termination_counter = 0;
        } else {
            // For even attempts, advance the chooser direction; otherwise alternate the pointer direction
            if self.state.termination_counter % 2 == 0 {
                self.state.chooser = self.state.chooser.next();
            } else {
                self.state.direction = self.state.direction.next();
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
    /// TODO: the return type can be strengthened to Color::Color without too much work
    fn next_coordinates(&self) -> Option<((usize, usize), &Color, bool)> {
        let first_edge = self.disjoint_edge_coordinate(self.state.pointer, self.state.direction);
        let second_edge = self
            .disjoint_edge_coordinate(first_edge, self.state.chooser.choose(self.state.direction));

        // Check if we're moving into:
        //  - either the edge of the program or a black codel in which case we stop
        //  - a white codel in which case we:
        //    - find and move to the first edge and then stop (even if there are other non-contiguous ones later)
        //    - step into it if it's a colored codel, otherwise stay in the current white codel
        //  - a colored codel in which case we step one square into it and stop
        if let Some((next_location, next_color)) = self
            .program
            .maybe_next_point(second_edge, self.state.direction)
        {
            match next_color {
                Color::Black => None,
                Color::White => {
                    // Find the edge of the white region
                    let white_edge = self.edge_coordinate(next_location, self.state.direction);
                    // If we're about to step into a colored codel, do it; otherwise, stop at the edge
                    match self
                        .program
                        .maybe_next_point(white_edge, self.state.direction)
                    {
                        // TODO: ideally this guard would be written to just ensure that this IS colored rather than NOT everything else
                        Some((location, color))
                            if color != &Color::White && color != &Color::Black =>
                        {
                            Some((location, color, true))
                        }
                        _ => Some((white_edge, &Color::White, true)),
                    }
                }
                _ => Some((next_location, next_color, false)),
            }
        } else {
            None
        }
    }

    /// The coordinate of the closest region edge (exclusive) reached starting from `start` and moving in `direction`.
    fn edge_coordinate(&self, start: (usize, usize), direction: Direction) -> (usize, usize) {
        let color = self.program.color_at(start);
        let mut pointer = start;
        let mut edge = false;
        while !edge {
            match self.program.maybe_next_point(pointer, direction) {
                Some((next_pointer, next_color)) if next_color == color => pointer = next_pointer,
                _ => edge = true,
            }
        }
        pointer
    }

    /// The coordinate of the farthest region edge (exclusive) reached starting from `start` and moving in `direction`.
    fn disjoint_edge_coordinate(
        &self,
        start: (usize, usize),
        direction: Direction,
    ) -> (usize, usize) {
        self.program.region_at(start).edge(start, direction.into())
    }
}
