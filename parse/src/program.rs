extern crate image;
extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use image::GenericImageView;
use itertools::Itertools;

use crate::codel::Codel;
use crate::region::Region;

/// A Piet program is represented as a 2d grid of Codels.
///
/// # Parameters
///
/// * `codels` - A collection of rows of Codels where `codels[0][0]` represents the top-left Codel.
/// * `rows` - The number of rows in the program.
/// * `cols` - The number of cols in the program.
#[derive(Debug)]
pub struct Program {
    region_map: HashMap<(usize, usize), usize>,
    regions: Vec<Region>,
    codels: Vec<Vec<Codel>>,
    rows: usize,
    cols: usize,
}

impl Program {
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

        let (regions, region_map) = Self::get_regions(&codels, &rows, &cols);

        Program {
            region_map,
            regions,
            codels,
            cols,
            rows,
        }
    }

    fn get_regions(codels: &Vec<Vec<Codel>>, rows: &usize, cols: &usize) -> (Vec<Region>, HashMap<(usize, usize), usize>) {
        // All of the codels that have already been incorporated into regions
        let mut seen = HashSet::new();
        // All of the seen regions
        let mut regions = Vec::new();
        // Maps codel coordinates to the index of the region they belong to
        let mut region_map = HashMap::new();
        for row in 0..*rows {
            for col in 0..*cols {
                let new = !seen.contains(&(row, col));
                let colored = match &codels[row][col] {
                    Codel::Color { .. } => true,
                    Codel::Black | Codel::White => false,
                };
                // Build a region if the codel hasn't been seen and it has a color
                if new && colored {
                    let (region, coordinates) = Self::get_region(codels, (row, col));
                    regions.push(region);
                    let region_idx = regions.len();
                    for point in coordinates {
                        seen.insert(point);
                        region_map.insert(point, region_idx);
                    }
                }
            }
        }
        (regions, region_map)
    }

    /// Build a contiguous region of the same color
    fn get_region(codels: &Vec<Vec<Codel>>, point: (usize, usize)) -> (Region, HashSet<(usize, usize)>) {
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

        // TODO: there should be an easy way to ditch this clone
        (Region::new(Vec::from_iter(members.clone())), members)
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
}
