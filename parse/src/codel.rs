use crate::color::Color;
use crate::region::Region;

#[derive(Debug, PartialEq)]
pub struct Codel {
    pub color: Color,
    pub region: Region,
}
