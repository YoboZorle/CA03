use ca03::game::{
    self,
    block,
    overlay::{Grid, Overlay},
    settings,
    world::World,
    Drawable,
};
use piston_window::{
    self,
    clear,
    AdvancedWindow,
    Button,
    Key,
    OpenGL,
    PistonWindow,
    ResizeEvent,
    WindowSettings,
    *,
};
use sdl2_window::Sdl2Window;
use std::{cell::RefCell, rc::Rc};
extern crate find_folder;

fn main() {
    // block::step([1; 16], 257, vec![2]);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let opengl = OpenGL::V4_5;
    let title = "CA03)";
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title,
    [800, 600])     .exit_on_esc( settings::<bool>("exit_on_esc") )
        // .samples(16)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(settings::<bool>("capture_cursor"));
    window.set_max_fps(settings::<u64>("framerate"));
    window.set_ups(2 * settings::<u64>("framerate"));
    let mut cursor = [0.0, 0.0];
    let mut world = World::default();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    let dim = Rc::new(RefCell::new({
        let Size { width, height } = window.window.draw_size();
        (width, height)
    }));
    let mut ar = dim.borrow().0 / dim.borrow().1;
    let mut ups = 0.;
    world.grid =
        Grid::new(true, Some(ar), settings::<f64>("grid_size"), dim.clone());
    let mut stats = false;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            if !stats {
                world.fps.tick();
            }
            clear([0.0, 0.0, 0.0, 1.0], g);
            world.draw(world.grid.size() as f64, dim.clone(), c, g);
            let d = dim.borrow();
            if stats {
                text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
                    .draw(
                        &world.fps.tick().to_string(),
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(d.0 - 34., 17.0),
                        g,
                    )
                    .unwrap();
                text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
                    .draw(
                        &(ups as u32).to_string(),
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(d.0 - 34., 36.0),
                        g,
                    )
                    .unwrap();
            }
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
        e.mouse_cursor(|pos| {
            cursor = pos;
        });
        e.mouse_scroll(|d| {
            &world.grid.set_size(&world.grid.size() + 2 * d[1] as i32);
        });
        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left {
                world.add(Grid::get_pos(&world.grid, cursor[0], cursor[1]));
            }
            if button == MouseButton::Right {
                world.remove(world.grid.get_pos(cursor[0], cursor[1]));
            }
        }

        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;

            if button == Keyboard(Key::W) {
                &world.grid.toggle();
            } else if button == Keyboard(Key::R) {
                &world.grid.set_size(20);
                stats = !stats;
            } else if button == Keyboard(Key::S) {
                stats = !stats;
            }
        }
        if let Some(_) = e.resize_args() {
            let Size { width, height } = window.window.draw_size();
            dim.replace((width, height));
            ar = width / height;
            &world.grid.set_ar(ar);
        }
        if let Some(_args) = e.idle_args() {
            // println!("{}", args.dt);
        }
        if let Some(args) = e.update_args() {
            ups = 1. / args.dt;
            // println!("{}", args.dt);
            // println!("{}", fps.tick());
            world.update();
        }
    }
}
