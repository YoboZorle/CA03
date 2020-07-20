use super::{overlay::Overlay, Drawable};
use gfx_device_gl::CommandBuffer;
use gfx_device_gl::Resources;
use gfx_graphics::GfxGraphics;
use na::Point4;
use nalgebra as na;
use piston_window::{rectangle, Context};
use rectangle::square;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct World {
    origin: Point4<u64>,
    entities: HashMap<Point4<i32>, i32>,
    overlays: Vec<Box<dyn Overlay>>,
}
impl Default for World {
    fn default() -> Self {
        let origin = Point4::<u64>::new(0, 0, 0, 0);
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
        size: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        let s = screen.borrow();
        for i in self.entities.keys() {
            let cellsize = s.0 / size;
            rectangle(
                [1., 0., 0., 1.],
                [
                    (s.0 / 2. + i.x as f64 * cellsize),
                    (s.1 / 2. + i.y as f64 * cellsize),
                    cellsize,
                    cellsize,
                ],
                c.transform,
                g,
            );
        }
    }
}
impl World {
    pub fn new() -> Self {
        let origin = Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = vec![];
        Self {
            origin,
            entities,
            overlays,
        }
    }

    pub fn add(&mut self, pos: (i32, i32)) {
        self.entities.insert(Point4::new(pos.0, pos.1, 0, 0), 0);
    }
    pub fn remove(&mut self, pos: (i32, i32)) {
        self.entities.remove(&Point4::new(pos.0, pos.1, 0, 0));
    }
}
