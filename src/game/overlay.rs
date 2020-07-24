use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::{line, Context};

use super::Drawable;
use std::{cell::RefCell, rc::Rc};
pub trait Overlay {
    fn is_exclusive(&self) -> bool;
    fn size(&self) -> i32;
    fn toggle(&mut self);
    fn draw(
        &self,
        ar: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    );
}

#[derive(Default)]
pub struct Grid {
    show:      bool,
    cellcount: f64,
    ar:        f64,
    dim:       Rc<RefCell<(f64, f64)>>,
}

impl Drawable for Grid {
    fn draw(
        &self,
        ar: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        if self.show {
            let screen = screen.borrow();
            let width = screen.0 / self.cellcount;
            let height = screen.1 / self.cellcount * ar;
            (0..self.cellcount as i32 + 1)
                .map(|i| {
                    let cs =
                        screen.1 / 2. + (i as f64 - self.cellcount / 2.) * height;
                    ([i as f64 * width, 0., i as f64 * width, screen.1], [
                        0., cs, screen.0, cs,
                    ])
                })
                .for_each(|(v, h)| {
                    line([1.; 4], 1.0, v, c.transform, g);
                    line([1.; 4], 1.0, h, c.transform, g);
                });
        }
    }
}

impl Grid {
    pub fn new(
        show: bool,
        ar: Option<f64>,
        size: f64,
        dim: Rc<RefCell<(f64, f64)>>,
    ) -> Self {
        let show = show;
        let ar = ar.unwrap_or(1.);
        let cellcount = size;
        Self {
            show,
            ar,
            cellcount,
            dim,
        }
    }

    pub fn get_pos(
        &self,
        x: f64,
        y: f64,
    ) -> (i32, i32) {
        let d = self.dim.borrow();
        (
            ((x - d.0 / 2.) / (d.0 / self.cellcount)).floor() as i32,
            ((y - d.1 / 2.) / d.1 * (self.cellcount / self.ar)).floor() as i32,
        )
    }

    pub fn set_size(
        &mut self,
        s: i32,
    ) -> &mut Self {
        self.cellcount = s as f64;
        self
    }

    pub fn set_ar(
        &mut self,
        ar: f64,
    ) -> &mut Self {
        self.ar = ar as f64;
        self
    }
}

impl Overlay for Grid {
    fn is_exclusive(&self) -> bool { true }

    fn size(&self) -> i32 { self.cellcount.abs() as i32 }

    fn toggle(&mut self) { self.show = !self.show; }

    fn draw(
        &self,
        ar: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        Drawable::draw(self, ar, screen, c, g);
    }
}
