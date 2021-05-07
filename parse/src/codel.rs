use image::{Rgba, Pixel};

/// Hue cycle:
/// - red -> yellow -> green -> cyan -> blue -> magenta -> red
/// -  0  ->  1     ->  2    ->  3   ->  4   ->  5      ->  0
///
/// Lightness cycle:
/// - light -> normal -> dark -> light
/// -  0    ->  1     ->  2   -> 0
#[derive(Debug)]
pub enum Codel {
    Color { hue: u8, lightness: u8},
    Black,
    White,
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
