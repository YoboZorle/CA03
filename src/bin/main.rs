use piston_window::{
    self,
    clear,
    ellipse,
    rectangle,
    AdvancedWindow,
    Button,
    Event,
    Key,
    OpenGL,
    PistonWindow,
    RenderArgs,
    RenderEvent,
    ResizeEvent,
    WindowSettings,
};

use sdl2_window::Sdl2Window;
use std::{cell::RefCell, rc::Rc};

use piston_window::*;

use ca03::game::{
    overlay::{Grid, Overlay},
    world::World,
    Drawable,
};

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

    let dim = Rc::new(RefCell::new({
        let Size { width, height } = window.window.draw_size();
        (width, height)
    }));
    let mut ar = dim.borrow().0 / dim.borrow().1;

    let mut world = World::default();
    let mut grid = Grid::new(true, 10., Some(ar), dim.clone());

    let mut cursor = [0.0, 0.0];

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            world.draw(grid.size() as f64, dim.clone(), c, g);
            grid.draw(ar, dim.clone(), c, g);
        });
        e.mouse_cursor(|pos| {
            cursor = pos;
        });
        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left {
                world.add(grid.get_pos(cursor[0], cursor[1]));
            }
            if button == MouseButton::Right {
                world.remove(grid.get_pos(cursor[0], cursor[1]));
            }
        }
        e.mouse_scroll(|d| {
            grid.set_size(grid.size() + 2 * d[1] as i32);
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
            let Size { width, height } = window.window.draw_size();
            *dim.borrow_mut() = (width, height);
            ar = width / height;
            grid.set_ar(ar);
        }
    }
}
