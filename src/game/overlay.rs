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
}

impl Overlay for Grid {
    fn size(&self) -> i32 {
        self.size as i32
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
    pub fn toggle(&mut self) {
        self.show = !self.show;
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
            for i in 0..((((screen.width - 1.) / self.size) as i32) + 1) {
                line(
                    [1.; 4],
                    1.0,
                    [
                        i as f64 * self.size,
                        0.,
                        i as f64 * self.size,
                        screen.height,
                    ],
                    c.transform,
                    g,
                );
            }
            for i in 0..((((screen.height - 1.) / self.size) as i32) + 1) {
                line(
                    [1.; 4],
                    1.0,
                    [0., i as f64 * self.size, screen.width, i as f64 * self.size],
                    c.transform,
                    g,
                );
            }
        }
    }
}
