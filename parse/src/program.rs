extern crate image;
extern crate itertools;

use std::collections::{HashMap, HashSet};

use image::GenericImageView;
use itertools::Itertools;

use crate::codel::Codel;
use crate::direction::ProgramDirection;

/// A Piet program is represented as a 2d grid of Codels.
///
/// # Parameters
///
/// * `codels` - A collection of rows of Codels where `codels[0][0]` represents the top-left Codel.
/// * `regions` - A map of coordinates to the size of their corresponding regions.
/// * `rows` - The number of rows in the program.
/// * `cols` - The number of cols in the program.
#[derive(Debug)]
pub struct Program {
    pub codels: Vec<Vec<Codel>>,
    pub regions: HashMap<(usize, usize), usize>,
    rows: usize,
    cols: usize,
}

impl Program {
    pub fn new(codels: Vec<Vec<Codel>>, rows: usize, cols: usize) -> Program {
        let regions = Self::get_regions(&codels, &rows, &cols);

        Program {
            regions,
            codels,
            rows,
            cols,
        }
    }

    /// Loads a program from a file given its path.
    pub fn load(path: &str) -> Program {
        let img = image::open(path).unwrap();
        let (cols, rows) = {
            let (r_cols, r_rows) = img.dimensions();
            (r_cols as usize, r_rows as usize)
        };
        let codels: Vec<Vec<Codel>> = img
            .pixels()
            .chunks(cols as usize)
            .into_iter()
            .map(|row| row.map(|(.., color)| Codel::from(color)).collect())
            .collect();

        Self::new(codels, rows, cols)
    }

    /// Builds a map of program coordinates to the sizes of their corresponding regions
    fn get_regions(codels: &Vec<Vec<Codel>>, rows: &usize, cols: &usize) -> HashMap<(usize, usize), usize> {
        // Maps codel coordinates to the index of the size of the region that they belong to
        let mut regions = HashMap::new();
        for row in 0..*rows {
            for col in 0..*cols {
                // Build a region if the codel hasn't been seen before
                if !regions.contains_key(&(row, col)) {
                    let members = Self::get_region(codels, (row, col));
                    let count = members.len();
                    for point in members {
                        regions.insert(point, count);
                    }
                }
            }
        }
        regions
    }

    /// Get all members of the same contiguous region of color
    fn get_region(codels: &Vec<Vec<Codel>>, point: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut seen = HashSet::new();
        let mut members = HashSet::new();
        let mut neighbors = vec![point];

        for &neighbor in &neighbors {
            seen.insert(neighbor);
        };

        let (row, col) = point;
        let codel = &codels[row][col];

        while let Some(neighbor) = neighbors.pop() {
            let (n_row, n_col) = neighbor;
            let n_codel = codels.get(n_row).and_then(|row| row.get(n_col));
            match n_codel {
                Some(same) if same == codel => {
                    members.insert(neighbor);
                    for n_neighbor in Self::neighbors(neighbor) {
                        if !seen.contains(&n_neighbor) {
                            seen.insert(neighbor);
                            neighbors.push(n_neighbor);
                        }
                    }
                }
                _ => ()
            }
        }

        members
    }

    /// Get all the neighbors of a given point
    fn neighbors(point: (usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = point;
        // TODO: this is really gross, maybe just give up and use isizes instead
        let mut neighbors = match (row.checked_sub(1), col.checked_sub(1)) {
            (Some(row_sub), Some(col_sub)) => vec![(row_sub, col), (row, col_sub)],
            (Some(row_sub), None) => vec![(row_sub, col)],
            (None, Some(col_sub)) => vec![(row, col_sub)],
            (None, None) => vec![],
        };
        neighbors.push((row + 1, col));
        neighbors.push((row, col + 1));
        neighbors
    }

    /// Gets the next codel along with its coordinates if one exists.
    pub fn maybe_next_codel<T: Into<ProgramDirection>>(&self, start: (usize, usize), direction: T) -> Option<((usize, usize), &Codel)> {
        let (row, col) = start;
        let maybe_next = match direction.into() {
            ProgramDirection::Up => row
                .checked_sub(1)
                .map(|next_row| (next_row, col)),
            ProgramDirection::Down => {
                let next_row = row + 1;
                if next_row < self.rows {
                    Some((next_row, col))
                } else {
                    None
                }
            }
            ProgramDirection::Left => col
                .checked_sub(1)
                .map(|next_col| (row, next_col)),
            ProgramDirection::Right => {
                let next_col = col + 1;
                if next_col < self.cols {
                    Some((row, next_col))
                } else {
                    None
                }
            }
        };
        maybe_next.map(|next| (next, self.codel_at(next)))
    }

    /// Gets the codel at the specified (row, column) point.
    pub fn codel_at(&self, point: (usize, usize)) -> &Codel {
        let (row, col) = point;
        &self.codels[row][col]
    }
}

#[cfg(test)]
mod test_program {
    use super::*;

    #[test]
    fn test_regions() {
        let codels = vec![
            vec![Codel::White, Codel::White, Codel::Black],
            vec![Codel::White, Codel::Black, Codel::Black],
            vec![Codel::White, Codel::Black, Codel::Black],
        ];
        let program = Program::new(codels, 3, 3);
        let expected = vec![
            ((0, 0), 4), ((0, 1), 4), ((0, 2), 5),
            ((1, 0), 4), ((1, 1), 5), ((1, 2), 5),
            ((2, 0), 4), ((2, 1), 5), ((2, 2), 5),
        ];
        assert_eq!(program.regions, expected.into_iter().collect());
    }

    #[test]
    fn test_maybe_next_codel() {
        let codels = vec![
            vec![Codel::Color { hue: 0, lightness: 0 }, Codel::Color { hue: 0, lightness: 1 }, Codel::Color { hue: 0, lightness: 2 }],
            vec![Codel::Color { hue: 1, lightness: 0 }, Codel::Color { hue: 1, lightness: 1 }, Codel::Color { hue: 1, lightness: 2 }],
            vec![Codel::Color { hue: 2, lightness: 0 }, Codel::Color { hue: 2, lightness: 1 }, Codel::Color { hue: 2, lightness: 2 }],
        ];
        let program = Program::new(codels, 3, 3);

        // corners
        assert_eq!(program.maybe_next_codel((0, 0), ProgramDirection::Up), None);
        assert_eq!(program.maybe_next_codel((0, 0), ProgramDirection::Left), None);
        assert_eq!(program.maybe_next_codel((2, 2), ProgramDirection::Right), None);
        assert_eq!(program.maybe_next_codel((2, 2), ProgramDirection::Down), None);

        // corners
        assert_eq!(program.maybe_next_codel((1, 1), ProgramDirection::Up), Some(((0, 1), &Codel::Color { hue: 0, lightness: 1 })));
        assert_eq!(program.maybe_next_codel((1, 1), ProgramDirection::Left), Some(((1, 0), &Codel::Color { hue: 1, lightness: 0 })));
        assert_eq!(program.maybe_next_codel((1, 1), ProgramDirection::Right), Some(((1, 2), &Codel::Color { hue: 1, lightness: 2 })));
        assert_eq!(program.maybe_next_codel((1, 1), ProgramDirection::Down), Some(((2, 1), &Codel::Color { hue: 2, lightness: 1 })));
    }

    #[test]
    fn test_codel_at() {
        let codels = vec![
            vec![Codel::White, Codel::White],
            vec![Codel::White, Codel::Black]
        ];
        let program = Program::new(codels, 2, 2);
        assert_eq!(program.codel_at((1, 1)), &Codel::Black);
    }
}
