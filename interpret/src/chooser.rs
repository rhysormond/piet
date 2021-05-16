use parse::direction::Direction;

/// Possible directions for the direction chooser.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Chooser {
    Left,
    Right,
}

impl Chooser {
    /// The next/opposite direction
    pub fn next(&self) -> Self {
        match self {
            Chooser::Left => Chooser::Right,
            Chooser::Right => Chooser::Left,
        }
    }
    /// The direction after applying a chooser direction
    pub fn choose(&self, direction: Direction) -> Direction {
        match self {
            Chooser::Left => direction.previous(),
            Chooser::Right => direction.next(),
        }
    }
}
