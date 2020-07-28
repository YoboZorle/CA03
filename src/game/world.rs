use super::{
    block::Block,
    overlay::{Grid, Overlay},
    settings,
    Drawable,
};
use fps_counter::FPSCounter;
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
extern crate fps_counter;

pub struct World {
    origin:    Point4<u64>,
    entities:  HashMap<Point4<i32>, Block>,
    overlays:  HashMap<u32, Box<dyn Overlay>>,
    exclusive: HashSet<TypeId>,
    pub grid:  Grid,
    pub fps:   FPSCounter,
    ups:       f64,
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

        self.entities.values().for_each(|a| {
            rectangle(
                [
                    (a.filled() / size) as f32 * a.pos().x.abs() as f32,
                    (a.filled() / size) as f32 * a.pos().y.abs() as f32,
                    (a.filled() / size) as f32 * a.pos().z.abs() as f32,
                    a.filled() as f32,
                ],
                [
                    (s.0 / 2. + a.pos().x as f64 * cellsize),
                    (s.1 / 2. + a.pos().y as f64 * cellsize),
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
impl Default for World {
    fn default() -> Self {
        let origin = Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = HashMap::new();
        let exclusive = HashSet::new();
        let grid = Grid::default();
        let fps = fps_counter::FPSCounter::new();
        let ups = 2. * settings::<f64>("framerate");

        Self {
            origin,
            entities,
            overlays,
            exclusive,
            grid,
            fps,
            ups,
        }
    }
}
impl World {
    pub fn new() -> Self {
        let origin = Point4::<u64>::new(0, 0, 0, 0);
        let entities = HashMap::new();
        let overlays = HashMap::new();
        let exclusive = HashSet::new();
        let grid = Grid::default();
        let fps = fps_counter::FPSCounter::new();
        let ups = 2. * settings::<f64>("framerate");

        Self {
            origin,
            entities,
            overlays,
            exclusive,
            grid,
            fps,
            ups,
        }
    }

    pub fn add(
        &mut self,
        pos: (i32, i32),
    ) {
        let loc = Point4::new(pos.0, pos.1, 0, 0);
        self.entities.insert(
            loc,
            Block::new(
                loc,
                Point4::new(0., 1., 1. / 10. * self.ups, 1.),
                0u128,
                vec![],
                vec![],
                Some(false),
                Some(false),
                Some(true),
            ),
        );
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

    pub fn update(&mut self) {
        self.entities.iter_mut().for_each(|(_, v)| v.update());
    }
}
