extern crate piston;
extern crate opengl_graphics;
extern crate graphics;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{ Context, Graphics };
use std::collections::HashMap;
use piston::window::{ AdvancedWindow, Window, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: AppWindow = WindowSettings::new("Pizarra", [300, 300])
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let ref mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut is_drawing = false;
    let mut line: Vec<(f64, f64)> = Vec::new();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                graphics::clear([1.0; 4], graphics);

                for item in line.iter() {
                    graphics::ellipse(
                        [0.0, 0.0, 0.0, 0.7],
                        graphics::ellipse::circle(item.0, item.1, 2.0),
                        context.transform,
                        graphics
                    );
                }
            });
        }

        // button handling
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            is_drawing = true;
            line = Vec::new();
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            is_drawing = false;
        }

        event.mouse_cursor(|x, y| {
            if is_drawing {
                line.push((x, y));
            }
        });
    }
}
