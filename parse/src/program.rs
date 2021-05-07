extern crate image;

use std::collections::HashMap;
use image::GenericImageView;
use crate::codel::Codel;

pub struct Program {
    // source: HashMap<(u32, u32), >
}

impl Program {
    pub fn load(path: &str) -> image::DynamicImage {
        let img = image::open(path).unwrap();

        let result: HashMap<(u32, u32), Codel> = img.pixels().into_iter().map( | (x, y, color) | {
            ((x, y), Codel::from(color))
        }).collect();
        println!("{:?}", result);
        img
        //     .map | (x, y, pixel) | {
        //     pixel
        // }
    }

}
