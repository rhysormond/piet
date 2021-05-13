extern crate image;
extern crate itertools;

use image::GenericImageView;
use itertools::Itertools;

use crate::codel::Codel;

/// A Piet program is represented as a 2d grid of Codels.
///
/// # Parameters
///
/// * `codels` - A collection of rows of Codels where `codels[0][0]` represents the top-left Codel.
/// * `rows` - The number of rows in the program.
/// * `cols` - The number of cols in the program.
#[derive(Debug)]
pub struct Program {
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

        Program {
            codels,
            cols,
            rows,
        }
    }
}
