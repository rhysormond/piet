use std::collections::HashMap;

/// A contiguous region of the same color within the program
///
/// # Parameters
///
/// * `row_edges` - Maps a row number to the col indices of its (left, right) edges.
/// * `col_edges` - Maps a col number to the row indices of its (bottom, top) edges.
/// * `size` - The number of codels in the region.
#[derive(Debug)]
pub struct Region {
    row_edges: HashMap<usize, (usize, usize)>,
    col_edges: HashMap<usize, (usize, usize)>,
    pub(crate) size: usize,
}

impl Region {
    pub fn new(codels: Vec<(usize, usize)>) -> Self {
        let row_edges = {
            codels
                .group_by(|(p_row, _), (row, _)| p_row == row)
                .map(|row| {
                    let (idx, _) = row.first().unwrap();
                    let cols: Vec<&usize> = row
                        .iter()
                        .map(|(_, col)| col)
                        .collect();
                    let min = *cols.iter().min().unwrap();
                    let max = *cols.iter().max().unwrap();
                    (idx.clone(), (min.clone(), max.clone()))
                })
                .collect()
        };
        let col_edges = {
            codels
                .group_by(|(_, p_col), (_, col)| p_col == col)
                .map(|col| {
                    let (_, idx) = col.first().unwrap();
                    let rows: Vec<&usize> = col
                        .iter()
                        .map(|(row, _)| row)
                        .collect();
                    let min = *rows.iter().min().unwrap();
                    let max = *rows.iter().max().unwrap();
                    (idx.clone(), (min.clone(), max.clone()))
                })
                .collect()
        };
        Region {
            row_edges,
            col_edges,
            size: codels.len(),
        }
    }
}