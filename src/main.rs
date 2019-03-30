use opengl_graphics::{ GlGraphics, OpenGL };
use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;
use std::fs::File;
use std::io::Write;
use chrono::Local;

use pizarra::{App, Tool};

fn main() -> std::io::Result<()> {
    let opengl = OpenGL::V3_2;
    let initial_dimentions = [800.0, 400.0];

    // piston stuff
    let mut window: AppWindow = WindowSettings::new("Pizarra", initial_dimentions)
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let mut events = Events::new(EventSettings::new().lazy(true));

    let gl = GlGraphics::new(opengl);
    let mut app = App::new(gl, initial_dimentions);
    let mut ctrl_on = false;

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        match event.press_args() {
            Some(Button::Mouse(MouseButton::Left)) => app.start_drawing(),
            Some(Button::Mouse(MouseButton::Middle)) => app.start_moving(),
            Some(Button::Keyboard(Key::LCtrl)) | Some(Button::Keyboard(Key::RCtrl)) => { ctrl_on = true; },
            Some(Button::Keyboard(Key::Z)) => if ctrl_on {
                app.undo();
            },
            Some(Button::Keyboard(Key::R)) => if ctrl_on {
                app.set_tool(Tool::Rectangle);
            },
            Some(Button::Keyboard(Key::L)) => if ctrl_on {
                app.set_tool(Tool::Line);
            },
            _ => {},
        }

        match event.release_args() {
            Some(Button::Mouse(MouseButton::Left)) => app.finish_drawing(),
            Some(Button::Mouse(MouseButton::Middle)) => app.end_moving(),
            Some(Button::Keyboard(Key::LCtrl)) | Some(Button::Keyboard(Key::RCtrl)) => {
                ctrl_on = false;
            },
            _ => {},
        }

        event.mouse_cursor(|x, y| {
            app.handle_cursor([x, y]);
        });
        event.mouse_relative(|dx, dy| {
            app.handle_cursor_relative([dx, dy]);
        });
        event.mouse_scroll(|dx, dy| {
            app.delta_offset([dx, -dy]);
        });
        event.resize(|w, h| {
            app.resize([w, h]);
        });
    }

    // save the result file
    let filename = Local::now().format("livepresentation_%Y-%m-%dT%H-%M-%S.svg").to_string();
    let mut file = File::create(filename)?;

    file.write_all(&app.serialize().into_bytes())?;

    Ok(())
}
