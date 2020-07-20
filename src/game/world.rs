use super::Drawable;
use crate::game::Overlay;
use gfx_device_gl::CommandBuffer;
use gfx_device_gl::Resources;
use gfx_graphics::GfxGraphics;
use nalgebra as na;
use piston_window::{rectangle, Context};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct World {
    origin: na::Point4<u64>,
    entities: HashMap<na::Point4<i32>, i32>,
    overlays: Vec<Box<dyn Overlay>>,
}
impl Default for World {
    fn default() -> Self {
        let origin = na::Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = vec![];
        Self {
            origin,
            entities,
            overlays,
        }
    }
}
impl Drawable for World {
    fn draw(
        &self,
        ar: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        let s = screen.borrow();
        for i in self.entities.keys() {
            rectangle(
                [1., 0., 0., 1.],
                [
                    i.x as f64 * s.0 / ar,
                    i.y as f64 * s.0 / ar,
                    s.0 / ar,
                    s.0 / ar,
                ],
                c.transform,
                g,
            );
        }
    }
}
impl World {
    pub fn new() -> Self {
        let origin = na::Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = vec![];
        Self {
            origin,
            entities,
            overlays,
        }
    }

    pub fn add(&mut self, pos: (i32, i32)) {
        self.entities.insert(na::Point4::new(pos.0, pos.1, 0, 0), 0);
    }
    pub fn remove(&mut self, pos: (i32, i32)) {}
}
