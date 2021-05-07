extern crate image;

use std::collections::HashMap;
use image::GenericImageView;
use crate::codel::Codel;

#[derive(Debug)]
pub struct Program {
    codels: HashMap<(u32, u32), Codel>,
    height: u32,
    width: u32,
}

impl Program {
    pub fn load(path: &str) -> Program {
        let img = image::open(path).unwrap();

        let (width, height) = img.dimensions();

        let codels: HashMap<(u32, u32), Codel> = img.pixels().into_iter().map( | (x, y, color) | {
            ((x, y), Codel::from(color))
        }).collect();

        Program {
            codels,
            width,
            height
        }
    }
}