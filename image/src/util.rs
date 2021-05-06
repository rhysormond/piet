extern crate image;

pub fn load(path: &str) -> image::DynamicImage {
    image::open(path).unwrap()
}

