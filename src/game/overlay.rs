use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::line;
use piston_window::Context;
use piston_window::Window;

pub struct Grid {
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
impl Default for Grid {
    fn default() -> Self {
        let show = true;
        let size = 20.;
        let ar = 1.;
        Grid::new(show, size, Some(ar))
    }
}
impl Grid {
    pub fn new(show: bool, size: f64, ar: Option<f64>) -> Self {
        let show = show;
        let ar = ar.unwrap_or(1.);
        let size = size;
        Self { show, ar, size }
    }

    pub fn set_size(&mut self, s: i32) -> &mut Self {
        self.size = s as f64;
        self
    }
    pub fn update(
        &self,
        screen: window::Size,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        if self.show {
            let h = (screen.width / self.size) - 1. / self.size;
            let w = (screen.height / self.size) - 1. / self.size;
            (0..(self.size + 1.) as i32).for_each(|i| {
                line(
                    [1.; 4],
                    1.0,
                    [i as f64 * h, 0., i as f64 * h, screen.height],
                    c.transform,
                    g,
                );
                line(
                    [1.; 4],
                    1.0,
                    [0., i as f64 * w, screen.width, i as f64 * w],
                    c.transform,
                    g,
                );
            });
        }
    }
}
