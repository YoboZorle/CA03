use piston_window::{
    self, clear, ellipse, rectangle, AdvancedWindow, Button, Event, Key, OpenGL, PistonWindow,
    RenderArgs, RenderEvent, ResizeEvent, WindowSettings,
};

use sdl2_window::Sdl2Window;

use piston_window::*;

use ca03::game::overlay::Grid;
use ca03::game::world::World;

fn main() {
    let opengl = OpenGL::V4_5;
    let title = "CA03)";
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, [800, 600])
        .exit_on_esc(true)
        // .samples(16)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(false);
    let mut dim = window.window.draw_size();
    let mut ar = dim.width / dim.height;

    let world = World::new();
    let mut grid = Grid::default();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            grid.update(dim, c, g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [50.0, 50.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;

            if button == Keyboard(Key::W) {
                grid.toggle();
            } else if button == Keyboard(Key::R) {
                grid.set_size(80);
            } else if button == Keyboard(Key::E) {
                grid.set_size(20);
            }
        }
        if let Some(_) = e.resize_args() {
            dim = window.window.draw_size();
            ar = dim.width / dim.height;
        }
    }
}
