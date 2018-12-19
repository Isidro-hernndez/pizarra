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

type AxisValues = HashMap<(i32, u8), f64>;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("piston-example-user_input", [300, 600])
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let ref mut gl = GlGraphics::new(opengl);
    let mut cursor = [0.0, 0.0];

    let mut axis_values: AxisValues = HashMap::new();

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = event.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::C {
                println!("Turned capture cursor on");
                capture_cursor = !capture_cursor;
                window.set_capture_cursor(capture_cursor);
            }

            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(args) = event.button_args() {
            println!("Scancode {:?}", args.scancode);
        }
        if let Some(button) = event.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
                Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            }
        };
        if let Some(args) = event.controller_axis_args() {
            axis_values.insert((args.id, args.axis), args.position);
        }
        event.mouse_cursor(|x, y| {
            cursor = [x, y];
            println!("Mouse moved '{} {}'", x, y);
        });
        event.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        event.text(|text| println!("Typed '{}'", text));
        event.resize(|w, h| println!("Resized '{}, {}'", w, h));

        if let Some(cursor) = event.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse left"); }
        };

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);
                    draw_cursor(cursor, &window, &c, g);
                }
            );
        }

        if let Some(_args) = event.idle_args() {
            // println!("Idle {}", _args.dt);
        }

        if let Some(_args) = event.update_args() {
            /*
            // Used to test CPU overload.
            println!("Update {}", _args.dt);
            let mut x: f64 = 0.0;
            for _ in 0..500_000 {
                x += (1.0 + x).sqrt();
            }
            println!("{}", x);
            */
        }
    }
}

fn draw_cursor<G: Graphics>(
    cursor: [f64; 2],
    window: &Window,
    c: &Context,
    g: &mut G,
) {
    // Cursor.
    let cursor_color = [0.0, 0.0, 0.0, 1.0];

    graphics::ellipse(
        cursor_color,
        graphics::ellipse::circle(cursor[0], cursor[1], 4.0),
        c.transform,
        g
    );
}
