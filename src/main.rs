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

use pizarra::color::Color;
use pizarra::poly::{Shape, DrawCommand, Line};

fn main() {
    let opengl = OpenGL::V3_3;

    // more or less constant properties
    let mut window_width = 800.0;
    let mut window_height = 400.0;
    let thickness = 1.0;
    let mut offset = [window_width as f64/2.0, window_height as f64/2.0];
    let mut inv_offset = math::translate(math::mul_scalar(offset, -1.0));

    // piston stuff
    let mut window: AppWindow = WindowSettings::new("Pizarra", [window_width, window_height])
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let ref mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    // state
    let mut is_drawing = false;
    let mut is_moving = false;
    let mut objects: Vec<Box<dyn Shape>> = Vec::new();

    // Colors
    let bgcolor = Color::black().to_a();
    let guidecolor = Color::gray().to_a();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let t = math::multiply(c.transform, math::translate(offset));

                graphics::clear(bgcolor, g);

                // content
                for item in objects.iter() {
                    for cmd in item.draw_commands() {
                        match cmd {
                            DrawCommand::Line{
                                color, thickness, line,
                            } => graphics::line(color, thickness, line, t, g),
                        }
                    }
                }

                // UI
                graphics::line(
                    guidecolor,
                    thickness,
                    [-20.0, 0.0, 20.0, 0.0],
                    t, g
                );

                graphics::line(
                    guidecolor,
                    thickness,
                    [0.0, 20.0, 0.0, -20.0],
                    t, g
                );
            });
        }

        // Mouse Left Button pressed, start of drawing
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            is_drawing = true;
            objects.push(Box::new(Line::new()));
        }

        // Mouse Left Button released, end of drawing
        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            is_drawing = false;
        }

        // start of moving
        if let Some(Button::Mouse(MouseButton::Middle)) = event.press_args() {
            is_moving = true;
        }

        // end of moving
        if let Some(Button::Mouse(MouseButton::Middle)) = event.release_args() {
            is_moving = false;
        }

        // draw probably
        event.mouse_cursor(|x, y| {
            if is_drawing && !is_moving {
                if let Some(item) = objects.last_mut() {
                    item.handle(math::transform_pos(inv_offset, [x, y]));
                }
            }
        });

        // move canvas
        event.mouse_scroll(|dx, dy| {
            offset = math::add(offset, [dx, -dy]);
            inv_offset = math::translate(math::mul_scalar(offset, -1.0));
        });
        event.mouse_relative(|dx, dy| {
            if is_moving {
                offset = math::add(offset, [dx, dy]);
                inv_offset = math::translate(math::mul_scalar(offset, -1.0));
            }
        });
        event.resize(|w, h| {
            let dw = w - window_width;
            let dh = h - window_height;

            offset = math::add(offset, [dw/2.0, dh/2.0]);
            inv_offset = math::translate(math::mul_scalar(offset, -1.0));

            window_width = w;
            window_height = h;
        });
    }
}
