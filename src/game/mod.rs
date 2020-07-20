use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::Context;
use std::cell::RefCell;
use std::rc::Rc;

pub mod overlay;
pub mod world;

pub use overlay::Grid;
use overlay::Overlay;

pub trait Drawable {
    fn draw(
        &self,
        ar: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    );
}
