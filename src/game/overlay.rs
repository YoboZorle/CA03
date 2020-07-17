use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::line;
use piston_window::Context;
use piston_window::PistonWindow;
use piston_window::Size;
use piston_window::Window;
use sdl2_window::Sdl2Window;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Grid {
    dim: Rc<RefCell<(f64, f64)>>,
    show: bool,
    size: f64,
    ar: f64,
}

pub trait Overlay {
    // fn draw(&self);
    fn size(&self) -> i32;
    fn toggle(&mut self);
}

impl Overlay for Grid {
    fn size(&self) -> i32 {
        self.size as i32
    }
    fn toggle(&mut self) {
        self.show = !self.show;
    }
}
impl Grid {
    pub fn new(show: bool, size: f64, ar: Option<f64>, dim: Rc<RefCell<(f64, f64)>>) -> Self {
        let show = show;
        let ar = ar.unwrap_or(1.);
        let size = size;
        Self {
            show,
            ar,
            size,
            dim,
        }
    }

    pub fn get_pos(&self, x: f64, y: f64) -> (i32, i32) {
        let d = self.dim.borrow();
        (
            (x / d.0 * self.size * self.ar) as i32,
            ((y / d.1) * self.size) as i32,
        )
    }

    pub fn set_size(&mut self, s: i32) -> &mut Self {
        self.size = s as f64;
        self
    }
    pub fn set_ar(&mut self, ar: f64) -> &mut Self {
        self.ar = ar as f64;
        self
    }
    pub fn update(
        &self,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        if self.show {
            let screen = screen.borrow();
            let h = (screen.0 / self.size) / self.ar;
            let w = screen.1 / self.size;
            // println!("{} {} {}", self.ar, h, w);
            (0..(self.size + 1.) as i32)
                .map(|i| {
                    (
                        [i as f64 * h, 0., i as f64 * h, screen.1],
                        [0., i as f64 * w, screen.0, i as f64 * w],
                    )
                })
                .for_each(|(v, h)| {
                    line([1.; 4], 1.0, v, c.transform, g);
                    line([1.; 4], 1.0, h, c.transform, g);
                });
        }
    }
}
