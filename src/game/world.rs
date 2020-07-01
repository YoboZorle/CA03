use crate::game::Overlay;
use crate::game::Section;
use nalgebra as na;

pub struct World {
    origin: na::Point4<u64>,
    sect: Vec<Section>,
}
impl Default for World {
    fn default() -> Self {
        let origin = na::Point4::<u64>::new(0, 0, 0, 0);
        let sect = vec![Section::new()];
        Self { origin, sect }
    }
}
impl World {
    pub fn new() -> Self {
        let origin = na::Point4::<u64>::new(0, 0, 0, 0);
        let sect = vec![];
        Self { origin, sect }
    }
}
