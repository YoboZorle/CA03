use piston_window::{
    self, clear, ellipse, rectangle, AdvancedWindow, Button, Event, Key, OpenGL, PistonWindow,
    RenderArgs, RenderEvent, ResizeEvent, WindowSettings,
};

use sdl2_window::Sdl2Window;

use piston_window::*;

use ca03::game::overlay::Grid;
use ca03::game::overlay::Overlay;
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

    let mut cursor = [0.0, 0.0];

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
        e.mouse_cursor(|pos| {
            cursor = pos;
        });
        e.mouse_scroll(|d| {
            println!(
                "Scrolled mouse 'h:{}, v:{}', x:{} y:{}",
                d[0], d[1], cursor[0], cursor[1]
            );
            grid.set_size(grid.size() + d[1] as i32);
        });
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;

            if button == Keyboard(Key::W) {
                grid.toggle();
            } else if button == Keyboard(Key::R) {
                grid.set_size(20);
            }
        }
        if let Some(_) = e.resize_args() {
            dim = window.window.draw_size();
            ar = dim.width / dim.height;
        }
    }
}
