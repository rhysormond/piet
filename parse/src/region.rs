use std::collections::HashSet;

use itertools::Itertools;

use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub(crate) members: HashSet<(usize, usize)>,
    pub size: usize,
}

impl Region {
    pub fn new(members: HashSet<(usize, usize)>) -> Region {
        let size = members.len();
        Region { members, size }
    }

    /// The coordinate of the farthest region edge (exclusive) reached starting from `start` and moving in `direction`.
    /// TODO: these edges should probably be pre-computed
    pub fn edge(&self, start: (usize, usize), direction: Direction) -> (usize, usize) {
        let (row, col) = start;
        match direction {
            Direction::Up => (**self.codels_in_col(col).first().unwrap(), col),
            Direction::Down => (**self.codels_in_col(col).last().unwrap(), col),
            Direction::Left => (row, **self.codels_in_row(row).first().unwrap()),
            Direction::Right => (row, **self.codels_in_row(row).last().unwrap()),
        }
    }

    /// The rows of all codels in the same column in ascending order.
    fn codels_in_col(&self, col: usize) -> Vec<&usize> {
        self.members
            .iter()
            .filter_map(|(p_row, p_col)| if p_col == &col { Some(p_row) } else { None })
            .sorted()
            .collect()
    }

    /// The columns of all codels in the same row in ascending order.
    fn codels_in_row(&self, row: usize) -> Vec<&usize> {
        self.members
            .iter()
            .filter_map(|(p_row, p_col)| if p_row == &row { Some(p_col) } else { None })
            .sorted()
            .collect()
    }
}

#[cfg(test)]
mod test_region {
    use super::*;

    #[test]
    fn test_codels_in_row() {
        let region = Region::new(vec![(0, 0), (0, 1), (0, 2), (1, 0)].into_iter().collect());
        assert_eq!(region.codels_in_row(0), vec![&0, &1, &2]);
        assert_eq!(region.codels_in_row(1), vec![&0]);
    }

    #[test]
    fn test_codels_in_col() {
        let region = Region::new(vec![(0, 0), (0, 1), (1, 0), (2, 0)].into_iter().collect());
        assert_eq!(region.codels_in_col(0), vec![&0, &1, &2]);
        assert_eq!(region.codels_in_col(1), vec![&0]);
    }

    #[test]
    fn test_edge() {
        let region = Region::new(
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ]
            .into_iter()
            .collect(),
        );

        // Not disjoint
        assert_eq!(region.edge((0, 0), Direction::Down), (2, 0));
        assert_eq!(region.edge((0, 0), Direction::Right), (0, 2));
        assert_eq!(region.edge((2, 2), Direction::Up), (0, 2));
        assert_eq!(region.edge((2, 2), Direction::Left), (2, 0));

        // Disjoint
        assert_eq!(region.edge((0, 1), Direction::Down), (2, 1));
        assert_eq!(region.edge((1, 0), Direction::Right), (1, 2));
        assert_eq!(region.edge((2, 1), Direction::Up), (0, 1));
        assert_eq!(region.edge((1, 2), Direction::Left), (1, 0));
    }
}
