extern crate image;
extern crate itertools;

use std::collections::{HashMap, HashSet};

use image::GenericImageView;
use itertools::Itertools;

use crate::codel::Codel;
use crate::color::Color;
use crate::direction::Direction;
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
    pub points: Vec<Vec<Codel>>,
    rows: usize,
    cols: usize,
}

impl Program {
    pub fn new(colors: Vec<Vec<Color>>, rows: usize, cols: usize) -> Program {
        let points = Self::get_codels(&colors, &rows, &cols);

        Program { points, rows, cols }
    }

    /// Loads a program from a file given its path.
    pub fn load(path: &str) -> Program {
        let img = image::open(path).unwrap();
        let (cols, rows) = {
            let (r_cols, r_rows) = img.dimensions();
            (r_cols as usize, r_rows as usize)
        };
        let colors: Vec<Vec<Color>> = img
            .pixels()
            .chunks(cols as usize)
            .into_iter()
            .map(|row| row.map(|(.., color)| Color::from(color)).collect())
            .collect();

        Self::new(colors, rows, cols)
    }

    /// Builds a grid of Codels from a grid of Colors
    fn get_codels(colors: &[Vec<Color>], rows: &usize, cols: &usize) -> Vec<Vec<Codel>> {
        // Maps coordinates to the color region that they belong to
        let mut regions: HashMap<(usize, usize), Region> = HashMap::new();

        // TODO: there's a lot of gross cloning to get rid of here
        (0..*rows)
            .map(|row| {
                (0..*cols)
                    .map(|col| {
                        let location = (row, col);
                        let color = colors[row][col].clone();
                        // Build a region if the location hasn't been seen before
                        let region = if let Some(region) = regions.get(&location) {
                            region.clone()
                        } else {
                            let region = Self::get_region(colors, location);
                            for member in &region.members {
                                regions.insert(*member, region.clone());
                            }
                            region
                        };
                        Codel { color, region }
                    })
                    .collect()
            })
            .collect()
    }

    /// Get all members of the same contiguous region of color
    fn get_region(colors: &[Vec<Color>], point: (usize, usize)) -> Region {
        let mut members = HashSet::new();
        let mut neighbors = vec![point];
        let mut seen: HashSet<(usize, usize)> = neighbors.clone().into_iter().collect();

        let (row, col) = point;
        let color = &colors[row][col];

        while let Some(neighbor) = neighbors.pop() {
            let (n_row, n_col) = neighbor;
            let n_color = colors.get(n_row).and_then(|row| row.get(n_col));
            match n_color {
                Some(same) if same == color => {
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

    /// Gets the next color in the given direction along with its coordinates if one exists.
    pub fn next_point<T: Into<Direction>>(
        &self,
        start: (usize, usize),
        direction: T,
    ) -> Option<((usize, usize), &Color)> {
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
        maybe_next.map(|next| (next, self.color_at(next)))
    }

    /// Gets the codel at the specified (row, column) point.
    fn codel_at(&self, point: (usize, usize)) -> &Codel {
        let (row, col) = point;
        &self.points[row][col]
    }

    /// Gets the color at the specified (row, column) point.
    pub fn color_at(&self, point: (usize, usize)) -> &Color {
        &self.codel_at(point).color
    }

    /// Gets the region at the specified (row, column) point.
    pub fn region_at(&self, point: (usize, usize)) -> &Region {
        &self.codel_at(point).region
    }
}

#[cfg(test)]
mod test_program {
    use super::*;

    #[test]
    fn test_regions() {
        let colors = vec![
            vec![Color::White, Color::White],
            vec![Color::White, Color::Black],
        ];
        let program = Program::new(colors, 2, 2);
        let expected = vec![
            vec![
                Codel {
                    color: Color::White,
                    region: Region::new(vec![(0, 0), (0, 1), (1, 0)].into_iter().collect()),
                },
                Codel {
                    color: Color::White,
                    region: Region::new(vec![(0, 0), (0, 1), (1, 0)].into_iter().collect()),
                },
            ],
            vec![
                Codel {
                    color: Color::White,
                    region: Region::new(vec![(0, 0), (0, 1), (1, 0)].into_iter().collect()),
                },
                Codel {
                    color: Color::Black,
                    region: Region::new(vec![(1, 1)].into_iter().collect()),
                },
            ],
        ];
        assert_eq!(program.points, expected);
    }

    #[test]
    fn test_next_point() {
        let colors = vec![
            vec![
                Color::Color {
                    hue: 0,
                    lightness: 0,
                },
                Color::Color {
                    hue: 0,
                    lightness: 1,
                },
                Color::Color {
                    hue: 0,
                    lightness: 2,
                },
            ],
            vec![
                Color::Color {
                    hue: 1,
                    lightness: 0,
                },
                Color::Color {
                    hue: 1,
                    lightness: 1,
                },
                Color::Color {
                    hue: 1,
                    lightness: 2,
                },
            ],
            vec![
                Color::Color {
                    hue: 2,
                    lightness: 0,
                },
                Color::Color {
                    hue: 2,
                    lightness: 1,
                },
                Color::Color {
                    hue: 2,
                    lightness: 2,
                },
            ],
        ];
        let program = Program::new(colors, 3, 3);

        // corners
        assert_eq!(program.next_point((0, 0), Direction::Up), None);
        assert_eq!(program.next_point((0, 0), Direction::Left), None);
        assert_eq!(program.next_point((2, 2), Direction::Right), None);
        assert_eq!(program.next_point((2, 2), Direction::Down), None);

        // corners
        assert_eq!(
            program.next_point((1, 1), Direction::Up),
            Some((
                (0, 1),
                &Color::Color {
                    hue: 0,
                    lightness: 1
                }
            ))
        );
        assert_eq!(
            program.next_point((1, 1), Direction::Left),
            Some((
                (1, 0),
                &Color::Color {
                    hue: 1,
                    lightness: 0
                }
            ))
        );
        assert_eq!(
            program.next_point((1, 1), Direction::Right),
            Some((
                (1, 2),
                &Color::Color {
                    hue: 1,
                    lightness: 2
                }
            ))
        );
        assert_eq!(
            program.next_point((1, 1), Direction::Down),
            Some((
                (2, 1),
                &Color::Color {
                    hue: 2,
                    lightness: 1
                }
            ))
        );
    }

    #[test]
    fn test_color_at() {
        let colors = vec![
            vec![Color::White, Color::White],
            vec![Color::White, Color::Black],
        ];
        let program = Program::new(colors, 2, 2);
        assert_eq!(program.color_at((1, 1)), &Color::Black);
    }

    #[test]
    fn test_region_at() {
        let colors = vec![
            vec![Color::White, Color::White],
            vec![Color::White, Color::Black],
        ];
        let program = Program::new(colors, 2, 2);
        assert_eq!(
            program.region_at((1, 1)),
            &Region {
                members: vec![(1, 1)].into_iter().collect(),
                size: 1
            }
        );
    }
}
