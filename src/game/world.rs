use super::{
    overlay::{Grid, Overlay},
    Drawable,
};
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use na::Point4;
use nalgebra as na;
use piston_window::{rectangle, Context};
use std::{
    any::TypeId,
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct World {
    origin:    Point4<u64>,
    entities:  HashMap<Point4<i32>, i32>,
    overlays:  HashMap<u32, Box<dyn Overlay>>,
    exclusive: HashSet<TypeId>,
    pub grid:  Grid,
}
impl Default for World {
    fn default() -> Self {
        let origin = Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = HashMap::new();
        let exclusive = HashSet::new();
        let grid = Grid::default();
        Self {
            origin,
            entities,
            overlays,
            exclusive,
            grid,
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
        let cellsize = s.0 / size;
        let ar = s.0 / s.1;
        self.entities.keys().for_each(|a| {
            rectangle(
                [1., 0., 0., 1.],
                [
                    (s.0 / 2. + a.x as f64 * cellsize),
                    (s.1 / 2. + a.y as f64 * cellsize),
                    cellsize,
                    cellsize,
                ],
                c.transform,
                g,
            );
        });
        Drawable::draw(&self.grid, ar, screen.clone(), c, g);
        // self.overlays
        //     .values()
        //     .for_each(|a| a.draw(ar, screen.clone(), c, g))
        //     .for_each(drop);
    }
}
impl World {
    pub fn new() -> Self {
        let origin = Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = HashMap::new();
        let exclusive = HashSet::new();
        let grid = Grid::default();

        Self {
            origin,
            entities,
            overlays,
            exclusive,
            grid,
        }
    }

    pub fn add(
        &mut self,
        pos: (i32, i32),
    ) {
        self.entities.insert(Point4::new(pos.0, pos.1, 0, 0), 0);
    }

    pub fn remove(
        &mut self,
        pos: (i32, i32),
    ) {
        self.entities.remove(&Point4::new(pos.0, pos.1, 0, 0));
    }

    pub fn remove_overlay(
        &mut self,
        overlay: &u32,
    ) {
        self.overlays.remove(overlay);
    }

    pub fn add_overlay<T: 'static + Overlay + Overlay>(
        &mut self,
        overlay: T,
    ) {
        if overlay.is_exclusive() {
            if !self.exclusive.contains(&TypeId::of::<T>()) {
                self.exclusive.insert(TypeId::of::<T>());
            }
        } else {
            self.overlays
                .insert(self.overlays.len() as u32, Box::new(overlay));
        }
    }

    pub fn get_overlay(
        &mut self,
        n: &u32,
    ) -> Option<&Box<dyn Overlay>> {
        self.overlays.get(n)
    }

    pub fn overlays(
        &mut self
    ) -> std::collections::hash_map::Values<'_, u32, Box<dyn Overlay>> {
        self.overlays.values()
    }

    pub fn move_origin(
        &mut self,
        p: Point4<u64>,
    ) {
        self.origin += p.coords;
    }
}
