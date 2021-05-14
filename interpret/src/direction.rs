/// Possible directions for the direction pointer.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum PointerDirection {
    Up,
    Right,
    Down,
    Left,
}

impl PointerDirection {
    pub fn next(&self) -> Self {
        match self {
            PointerDirection::Up => PointerDirection::Right,
            PointerDirection::Right => PointerDirection::Down,
            PointerDirection::Down => PointerDirection::Left,
            PointerDirection::Left => PointerDirection::Up,
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
    pub fn next(&self) -> Self {
        match self {
            ChooserDirection::Left => ChooserDirection::Right,
            ChooserDirection::Right => ChooserDirection::Left,
        }
    }
}
