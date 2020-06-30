use piston_window::{
    self, clear, ellipse, rectangle, AdvancedWindow, Button, Event, Key, OpenGL, PistonWindow,
    RenderArgs, RenderEvent, ResizeEvent, WindowSettings,
};

use deform_grid::DeformGrid;
use sdl2_window::Sdl2Window;

use piston_window::*;

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

    let mut draw_grid = true;
    let mut grid = DeformGrid::new([0.0, 0.0, dim.width, dim.height], 24, 24);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            if draw_grid {
                // Draw grid.
                grid.draw_vertical_lines(
                    &Line::new([0.8, 0.8, 0.8, 1.0], 0.5),
                    &c.draw_state,
                    c.transform,
                    g,
                );
                grid.draw_horizontal_lines(
                    &Line::new([0.8, 0.8, 0.8, 1.0], 0.5),
                    &c.draw_state,
                    c.transform,
                    g,
                );
            }
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [50.0, 50.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;

            if button == Keyboard(Key::G) {
                draw_grid = !draw_grid;
                println!("Draw grid {}", draw_grid);
            } else if button == Keyboard(Key::R) {
                grid.reset_control_points();
                grid.reset_vertices_and_texture_coords();
                grid.update();
                println!("Reset grid");
            }
        }
        if let Some(_) = e.resize_args() {
            dim = window.window.draw_size();
            grid = DeformGrid::new([0.0, 0.0, dim.width, dim.height], 20, 20);
            grid.update();
        }
    }
}
