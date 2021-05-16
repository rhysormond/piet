/// Cardinal directions within the program.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// The next direction in clockwise order
    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    /// The previous direction in clockwise order
    pub fn previous(&self) -> Self {
        self.next().next().next()
    }
}
