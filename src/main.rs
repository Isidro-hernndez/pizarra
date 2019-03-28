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
use graphics::math;
use std::fs::File;
use std::io::Write;
use chrono::Local;

use pizarra::poly::DrawCommand;
use pizarra::{App, Pizarra, Tool};
use pizarra::serialize::Serialize;

fn main() -> std::io::Result<()> {
    let opengl = OpenGL::V3_2;

    // The host of everything
    let mut piz = Pizarra::new([800.0, 400.0]);

    // piston stuff
    let mut window: AppWindow = WindowSettings::new("Pizarra", piz.get_dimentions())
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    // the app
    let mut app = App::new(gl, piz);
    let mut ctrl_on = false;

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            app.start_drawing();
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            app.finish_drawing();
        }

        if let Some(Button::Mouse(MouseButton::Middle)) = event.press_args() {
            app.start_moving();
        }

        if let Some(Button::Mouse(MouseButton::Middle)) = event.release_args() {
            app.end_moving();
        }

        match event.press_args() {
            Some(Button::Keyboard(Key::LCtrl)) | Some(Button::Keyboard(Key::RCtrl)) => {
                ctrl_on = true;
            },
            _ => {},
        }
        match event.release_args() {
            Some(Button::Keyboard(Key::LCtrl)) | Some(Button::Keyboard(Key::RCtrl)) => {
                ctrl_on = false;
            },
            _ => {},
        }

        // ctrl-z
        if let Some(Button::Keyboard(Key::Z)) = event.press_args() {
            if ctrl_on {
                app.undo();
            }
        }

        // tool selection
        if let Some(Button::Keyboard(Key::R)) = event.press_args() {
            if ctrl_on {
                app.set_tool(Tool::Rectangle);
            }
        }
        if let Some(Button::Keyboard(Key::L)) = event.press_args() {
            if ctrl_on {
                app.set_tool(Tool::Line);
            }
        }

        // draw probably
        event.mouse_cursor(|x, y| {
            app.handle_cursor(x, y);
        });

        // move canvas
        event.mouse_scroll(|dx, dy| {
            app.update_offset(dx, -dy);
        });
        // or handle resize
        event.resize(|w, h| {
            app.resize(w, h);
        });
    }

    // save the result file
    let filename = Local::now().format("livepresentation_%Y-%m-%dT%H-%M-%S.svg").to_string();
    let mut file = File::create(filename)?;

    file.write_all(&app.serialize().into_bytes())?;

    Ok(())
}
