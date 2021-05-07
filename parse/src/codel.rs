use image::{Pixel, Rgba};

const HUE_CYCLE_SIZE: u8 = 3;
const LIGHTNESS_CYCLE_SIZE: u8 = 6;

/// Hue cycle:
/// - red -> yellow -> green -> cyan -> blue -> magenta -> red
/// -  0  ->  1     ->  2    ->  3   ->  4   ->  5      ->  0
///
/// Lightness cycle:
/// - light -> normal -> dark -> light
/// -  0    ->  1     ->  2   -> 0
#[derive(Debug)]
pub enum Codel {
    Color { hue: u8, lightness: u8 },
    Black,
    White,
}

impl Codel {
    /// Compares the hue or lightness change between
    fn compare_cyclic_values(current: &u8, next: &u8, cycle_size: &u8) -> u8 {
        // This is implemented with a checked_sub to avoid converting to/from i8 here
        let change = next.checked_sub(*current);
        change.unwrap_or_else(|| cycle_size - (current - next))
    }

    /// Returns (HueChange, LightnessChange)
    pub fn compare(&self, next: &Codel) -> Option<(u8, u8)> {
        match (self, next) {
            (Codel::Color { hue, lightness }, Codel::Color { hue: next_hue, lightness: next_lightness }) => {
                Some((
                    Codel::compare_cyclic_values(hue, next_hue, &HUE_CYCLE_SIZE),
                    Codel::compare_cyclic_values(lightness, next_lightness, &LIGHTNESS_CYCLE_SIZE)
                ))
            }
            _ => None,
        }
    }
}

impl From<Rgba<u8>> for Codel {
    fn from(from: Rgba<u8>) -> Self {
        match from.to_rgb().channels4() {
            (0xFF, 0xC0, 0xC0, ..) => Codel::Color { hue: 0, lightness: 0 },
            (0xFF, 0xFF, 0xC0, ..) => Codel::Color { hue: 1, lightness: 0 },
            (0xC0, 0xFF, 0xC0, ..) => Codel::Color { hue: 2, lightness: 0 },
            (0xC0, 0xFF, 0xFF, ..) => Codel::Color { hue: 3, lightness: 0 },
            (0xC0, 0xC0, 0xFF, ..) => Codel::Color { hue: 4, lightness: 0 },
            (0xFF, 0xC0, 0xFF, ..) => Codel::Color { hue: 5, lightness: 0 },
            (0xFF, 0x00, 0x00, ..) => Codel::Color { hue: 0, lightness: 1 },
            (0xFF, 0xFF, 0x00, ..) => Codel::Color { hue: 1, lightness: 1 },
            (0x00, 0xFF, 0x00, ..) => Codel::Color { hue: 2, lightness: 1 },
            (0x00, 0xFF, 0xFF, ..) => Codel::Color { hue: 3, lightness: 1 },
            (0x00, 0x00, 0xFF, ..) => Codel::Color { hue: 4, lightness: 1 },
            (0xFF, 0x00, 0xFF, ..) => Codel::Color { hue: 5, lightness: 1 },
            (0xC0, 0x00, 0x00, ..) => Codel::Color { hue: 0, lightness: 2 },
            (0xC0, 0xC0, 0x00, ..) => Codel::Color { hue: 1, lightness: 2 },
            (0x00, 0xC0, 0x00, ..) => Codel::Color { hue: 2, lightness: 2 },
            (0x00, 0xC0, 0xC0, ..) => Codel::Color { hue: 3, lightness: 2 },
            (0x00, 0x00, 0xC0, ..) => Codel::Color { hue: 4, lightness: 2 },
            (0xC0, 0x00, 0xC0, ..) => Codel::Color { hue: 5, lightness: 2 },
            (0xFF, 0xFF, 0xFF, ..) => Codel::White,
            (0x00, 0x00, 0x00, ..) => Codel::Black,
            (r, g, b, ..) => panic!("Unsupported color ({}, {}, {})!", r, g, b)
        }
    }
}

#[cfg(test)]
mod test_codel {
    use super::*;

    #[test]
    fn test_compare_cyclic_values_identical() {
        assert_eq!(Codel::compare_cyclic_values(&0, &0, &3), 0);
    }

    #[test]
    fn test_compare_cyclic_values_different() {
        assert_eq!(Codel::compare_cyclic_values(&0, &1, &3), 1);
    }

    #[test]
    fn test_compare_cyclic_values_cycled() {
        assert_eq!(Codel::compare_cyclic_values(&1, &0, &3), 2);
    }

    #[test]
    fn test_compare_colors() {
        let current = Codel::Color { hue: 0, lightness: 0 };
        let next = Codel::Color { hue: 1, lightness: 2 };
        assert_eq!(current.compare(&next), Some((1, 2)));
    }

    #[test]
    fn test_compare_not_colors() {
        let color = Codel::Color { hue: 0, lightness: 0 };
        assert_eq!(color.compare(&Codel::Black), None);
        assert_eq!(Codel::Black.compare(&color), None);
    }
}