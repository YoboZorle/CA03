use crate::game::Overlay;
use nalgebra as na;

pub struct Section {
    entities: Vec<na::Point4<i32>>,
    overlays: Vec<Box<dyn Overlay>>,
}

impl Default for Section {
    fn default() -> Self {
        let entities = vec![];
        let overlays = vec![];
        Self { entities, overlays }
    }
}

impl Section {
    pub fn new() -> Self {
        let entities = vec![];
        let overlays = vec![];
        Self { entities, overlays }
    }
    pub fn draw(&self) {}
}
