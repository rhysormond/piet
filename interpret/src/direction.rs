use parse::direction::ProgramDirection;

/// Possible directions for the direction pointer.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum PointerDirection {
    Up,
    Right,
    Down,
    Left,
}

impl PointerDirection {
    /// The next direction in clockwise order
    pub fn next(&self) -> Self {
        match self {
            PointerDirection::Up => PointerDirection::Right,
            PointerDirection::Right => PointerDirection::Down,
            PointerDirection::Down => PointerDirection::Left,
            PointerDirection::Left => PointerDirection::Up,
        }
    }

    /// The previous direction in clockwise order
    fn previous(&self) -> Self {
        self.next().next().next()
    }

    /// The direction after applying a chooser direction
    pub fn with_chooser(&self, chooser: ChooserDirection) -> Self {
        match chooser {
            ChooserDirection::Left => self.previous(),
            ChooserDirection::Right => self.next(),
        }
    }
}

impl From<PointerDirection> for ProgramDirection {
    fn from(from: PointerDirection) -> ProgramDirection {
        match from {
            PointerDirection::Up => ProgramDirection::Up,
            PointerDirection::Right => ProgramDirection::Right,
            PointerDirection::Down => ProgramDirection::Down,
            PointerDirection::Left => ProgramDirection::Left,
        }
    }
}

/// Possible directions for the direction chooser.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ChooserDirection {
    Left,
    Right,
}

impl ChooserDirection {
    /// The next/opposite direction
    pub fn next(&self) -> Self {
        match self {
            ChooserDirection::Left => ChooserDirection::Right,
            ChooserDirection::Right => ChooserDirection::Left,
        }
    }
}
