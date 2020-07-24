use config::{Config, File};
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::Context;
use serde::de::Deserialize;
use std::{cell::RefCell, rc::Rc, sync::RwLock};

pub mod block;
pub mod overlay;
pub mod world;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("Settings.toml")).unwrap();
        settings
    });
}
pub fn settings<'a, T: Deserialize<'a>>(key: &str) -> T {
    SETTINGS.read().unwrap().get::<T>(key).unwrap()
}
pub trait Drawable {
    fn draw(
        &self,
        ar: f64,
        screen: Rc<RefCell<(f64, f64)>>,
        c: Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    );
}
