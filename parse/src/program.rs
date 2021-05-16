extern crate image;
extern crate itertools;

use std::collections::{HashMap, HashSet};

use image::GenericImageView;
use itertools::Itertools;

use crate::codel::Codel;
use crate::direction::Direction;
use crate::point::Point;
use crate::region::Region;

/// A Piet program is represented as a 2d grid of Codels.
///
/// # Parameters
///
/// * `points` - A collection of rows of Codels and their corresponding Regions where `points[0][0]` represents the top-left point.
/// * `rows` - The number of rows in the program.
/// * `cols` - The number of cols in the program.
#[derive(Debug)]
pub struct Program {
    pub points: Vec<Vec<Point>>,
    rows: usize,
    cols: usize,
}

impl Program {
    pub fn new(codels: Vec<Vec<Codel>>, rows: usize, cols: usize) -> Program {
        let points = Self::get_points(&codels, &rows, &cols);

        Program { points, rows, cols }
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
    fn get_points(codels: &[Vec<Codel>], rows: &usize, cols: &usize) -> Vec<Vec<Point>> {
        // Maps codel coordinates to the index of the region that they belong to
        let mut regions: HashMap<(usize, usize), Region> = HashMap::new();

        // TODO: there's a lot of gross cloning to get rid of here
        (0..*rows)
            .map(|row| {
                (0..*cols)
                    .map(|col| {
                        let location = (row, col);
                        let codel = codels[row][col].clone();
                        // Build a region if the codel hasn't been seen before
                        let region = if let Some(region) = regions.get(&location) {
                            region.clone()
                        } else {
                            let region = Self::get_region(codels, location);
                            for member in &region.members {
                                regions.insert(*member, region.clone());
                            }
                            region
                        };
                        Point { codel, region }
                    })
                    .collect()
            })
            .collect()
    }

    /// Get all members of the same contiguous region of color
    fn get_region(codels: &[Vec<Codel>], point: (usize, usize)) -> Region {
        let mut seen = HashSet::new();
        let mut members = HashSet::new();
        let mut neighbors = vec![point];

        for &neighbor in &neighbors {
            seen.insert(neighbor);
        }

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
                _ => (),
            }
        }

        Region::new(members)
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
    pub fn maybe_next_codel<T: Into<Direction>>(
        &self,
        start: (usize, usize),
        direction: T,
    ) -> Option<((usize, usize), &Codel)> {
        let (row, col) = start;
        let maybe_next = match direction.into() {
            Direction::Up => row.checked_sub(1).map(|next_row| (next_row, col)),
            Direction::Down => {
                let next_row = row + 1;
                if next_row < self.rows {
                    Some((next_row, col))
                } else {
                    None
                }
            }
            Direction::Left => col.checked_sub(1).map(|next_col| (row, next_col)),
            Direction::Right => {
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
        &self.points[row][col].codel
    }

    /// Gets the region at the specified (row, column) point.
    pub fn region_at(&self, point: (usize, usize)) -> &Region {
        let (row, col) = point;
        &self.points[row][col].region
    }
}

#[cfg(test)]
mod test_program {
    use super::*;

    #[test]
    fn test_regions() {
        let codels = vec![
            vec![Codel::White, Codel::White],
            vec![Codel::White, Codel::Black],
        ];
        let program = Program::new(codels, 2, 2);
        let expected = vec![
            vec![
                Point {
                    codel: Codel::White,
                    region: Region::new(vec![(0, 0), (0, 1), (1, 0)].into_iter().collect()),
                },
                Point {
                    codel: Codel::White,
                    region: Region::new(vec![(0, 0), (0, 1), (1, 0)].into_iter().collect()),
                },
            ],
            vec![
                Point {
                    codel: Codel::White,
                    region: Region::new(vec![(0, 0), (0, 1), (1, 0)].into_iter().collect()),
                },
                Point {
                    codel: Codel::Black,
                    region: Region::new(vec![(1, 1)].into_iter().collect()),
                },
            ],
        ];
        assert_eq!(program.points, expected);
    }

    #[test]
    fn test_maybe_next_codel() {
        let codels = vec![
            vec![
                Codel::Color {
                    hue: 0,
                    lightness: 0,
                },
                Codel::Color {
                    hue: 0,
                    lightness: 1,
                },
                Codel::Color {
                    hue: 0,
                    lightness: 2,
                },
            ],
            vec![
                Codel::Color {
                    hue: 1,
                    lightness: 0,
                },
                Codel::Color {
                    hue: 1,
                    lightness: 1,
                },
                Codel::Color {
                    hue: 1,
                    lightness: 2,
                },
            ],
            vec![
                Codel::Color {
                    hue: 2,
                    lightness: 0,
                },
                Codel::Color {
                    hue: 2,
                    lightness: 1,
                },
                Codel::Color {
                    hue: 2,
                    lightness: 2,
                },
            ],
        ];
        let program = Program::new(codels, 3, 3);

        // corners
        assert_eq!(program.maybe_next_codel((0, 0), Direction::Up), None);
        assert_eq!(program.maybe_next_codel((0, 0), Direction::Left), None);
        assert_eq!(program.maybe_next_codel((2, 2), Direction::Right), None);
        assert_eq!(program.maybe_next_codel((2, 2), Direction::Down), None);

        // corners
        assert_eq!(
            program.maybe_next_codel((1, 1), Direction::Up),
            Some((
                (0, 1),
                &Codel::Color {
                    hue: 0,
                    lightness: 1
                }
            ))
        );
        assert_eq!(
            program.maybe_next_codel((1, 1), Direction::Left),
            Some((
                (1, 0),
                &Codel::Color {
                    hue: 1,
                    lightness: 0
                }
            ))
        );
        assert_eq!(
            program.maybe_next_codel((1, 1), Direction::Right),
            Some((
                (1, 2),
                &Codel::Color {
                    hue: 1,
                    lightness: 2
                }
            ))
        );
        assert_eq!(
            program.maybe_next_codel((1, 1), Direction::Down),
            Some((
                (2, 1),
                &Codel::Color {
                    hue: 2,
                    lightness: 1
                }
            ))
        );
    }

    #[test]
    fn test_codel_at() {
        let codels = vec![
            vec![Codel::White, Codel::White],
            vec![Codel::White, Codel::Black],
        ];
        let program = Program::new(codels, 2, 2);
        assert_eq!(program.codel_at((1, 1)), &Codel::Black);
    }

    #[test]
    fn test_region_at() {
        let codels = vec![
            vec![Codel::White, Codel::White],
            vec![Codel::White, Codel::Black],
        ];
        let program = Program::new(codels, 2, 2);
        assert_eq!(
            program.region_at((1, 1)),
            &Region {
                members: vec![(1, 1)].into_iter().collect(),
                size: 1
            }
        );
    }
}
