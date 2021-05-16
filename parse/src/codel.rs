use image::{Pixel, Rgba};

const HUE_CYCLE_SIZE: u8 = 6;
const LIGHTNESS_CYCLE_SIZE: u8 = 3;

/// A Piet program is composed of a grid of Codels.
///
/// Codels are one of black, white, or one of 18 different colors.
/// Codel colors are on a cyclic, 2d gradient of hue and lightness.
///
/// | hue/lightness | red     | yellow  | green   | cyan    | blue    | magenta |
/// |---------------|---------|---------|---------|---------|---------|---------|
/// | light         | #FFC0C0 | #FFFFC0 | #C0FFC0 | #C0FFFF | #C0C0FF | #FFC0FF |
/// | neutral       | #FF0000 | #FFFF00 | #00FF00 | #00FFFF | #0000FF | #FF00FF |
/// | dark          | #C00000 | #C0C000 | #00C000 | #00C0C0 | #0000C0 | #C000C0 |
/// along with black (`#000000`) and white (`#FFFFFF`)
///
/// The hue cycle is:
/// - red -> yellow -> green -> cyan -> blue -> magenta -> red
/// Which is represented internally as:
/// - 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0
///
/// The lightness cycle is:
/// - light -> neutral -> dark -> light
/// Which is represented internally as:
/// - 0 -> 1 -> 2 -> 0
#[derive(Debug, PartialEq, Clone)]
pub enum Codel {
    Color { hue: u8, lightness: u8 },
    Black,
    White,
}

impl Codel {
    /// Returns the distance between two values along a directed cycle.
    fn cyclic_distance(current: &u8, next: &u8, cycle_size: &u8) -> u8 {
        let change = next.checked_sub(*current);
        change.unwrap_or_else(|| cycle_size - (current - next))
    }

    /// Returns compares two Codels' hues and lightnesses and returns a tuple of (hueChange, lightnessChange)
    pub fn compare(&self, next: &Codel) -> Option<(u8, u8)> {
        match (self, next) {
            (
                Codel::Color { hue, lightness },
                Codel::Color {
                    hue: next_hue,
                    lightness: next_lightness,
                },
            ) => Some((
                Codel::cyclic_distance(hue, next_hue, &HUE_CYCLE_SIZE),
                Codel::cyclic_distance(lightness, next_lightness, &LIGHTNESS_CYCLE_SIZE),
            )),
            _ => None,
        }
    }
}

impl From<Rgba<u8>> for Codel {
    fn from(from: Rgba<u8>) -> Self {
        match from.to_rgb().channels4() {
            (0xFF, 0xC0, 0xC0, ..) => Codel::Color {
                hue: 0,
                lightness: 0,
            },
            (0xFF, 0xFF, 0xC0, ..) => Codel::Color {
                hue: 1,
                lightness: 0,
            },
            (0xC0, 0xFF, 0xC0, ..) => Codel::Color {
                hue: 2,
                lightness: 0,
            },
            (0xC0, 0xFF, 0xFF, ..) => Codel::Color {
                hue: 3,
                lightness: 0,
            },
            (0xC0, 0xC0, 0xFF, ..) => Codel::Color {
                hue: 4,
                lightness: 0,
            },
            (0xFF, 0xC0, 0xFF, ..) => Codel::Color {
                hue: 5,
                lightness: 0,
            },
            (0xFF, 0x00, 0x00, ..) => Codel::Color {
                hue: 0,
                lightness: 1,
            },
            (0xFF, 0xFF, 0x00, ..) => Codel::Color {
                hue: 1,
                lightness: 1,
            },
            (0x00, 0xFF, 0x00, ..) => Codel::Color {
                hue: 2,
                lightness: 1,
            },
            (0x00, 0xFF, 0xFF, ..) => Codel::Color {
                hue: 3,
                lightness: 1,
            },
            (0x00, 0x00, 0xFF, ..) => Codel::Color {
                hue: 4,
                lightness: 1,
            },
            (0xFF, 0x00, 0xFF, ..) => Codel::Color {
                hue: 5,
                lightness: 1,
            },
            (0xC0, 0x00, 0x00, ..) => Codel::Color {
                hue: 0,
                lightness: 2,
            },
            (0xC0, 0xC0, 0x00, ..) => Codel::Color {
                hue: 1,
                lightness: 2,
            },
            (0x00, 0xC0, 0x00, ..) => Codel::Color {
                hue: 2,
                lightness: 2,
            },
            (0x00, 0xC0, 0xC0, ..) => Codel::Color {
                hue: 3,
                lightness: 2,
            },
            (0x00, 0x00, 0xC0, ..) => Codel::Color {
                hue: 4,
                lightness: 2,
            },
            (0xC0, 0x00, 0xC0, ..) => Codel::Color {
                hue: 5,
                lightness: 2,
            },
            (0xFF, 0xFF, 0xFF, ..) => Codel::White,
            (0x00, 0x00, 0x00, ..) => Codel::Black,
            (r, g, b, ..) => panic!("Unsupported color ({}, {}, {})!", r, g, b),
        }
    }
}

#[cfg(test)]
mod test_codel {
    use super::*;

    #[test]
    fn test_cyclic_distance_identical() {
        assert_eq!(Codel::cyclic_distance(&0, &0, &3), 0);
    }

    #[test]
    fn test_cyclic_distance_different() {
        assert_eq!(Codel::cyclic_distance(&0, &1, &3), 1);
    }

    #[test]
    fn test_cyclic_distance_cycled() {
        assert_eq!(Codel::cyclic_distance(&1, &0, &3), 2);
    }

    #[test]
    fn test_compare_colors() {
        let current = Codel::Color {
            hue: 0,
            lightness: 0,
        };
        let next = Codel::Color {
            hue: 1,
            lightness: 2,
        };
        assert_eq!(current.compare(&next), Some((1, 2)));
    }

    #[test]
    fn test_compare_not_colors() {
        let color = Codel::Color {
            hue: 0,
            lightness: 0,
        };
        assert_eq!(color.compare(&Codel::Black), None);
        assert_eq!(Codel::Black.compare(&color), None);
    }
}
