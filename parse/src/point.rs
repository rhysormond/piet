use crate::codel::Codel;
use crate::region::Region;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub codel: Codel,
    pub region: Region,
}
