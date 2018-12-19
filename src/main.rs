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

struct Line {
    points: Vec<(f64, f64)>,
    drawn: bool,
}

impl Line {
    fn new() -> Line {
        Line {
            points: Vec::with_capacity(1000),
            drawn: false,
        }
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn push(&mut self, val: (f64, f64)) {
        self.points.push(val);
    }
}

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: AppWindow = WindowSettings::new("Pizarra", [300, 300])
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let ref mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut is_drawing = false;
    let mut lines: Vec<Line> = Vec::new();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                for line in lines.iter() {
                    if !line.drawn {
                        for item in line.points.iter() {
                            graphics::ellipse(
                                [0.5, 0.5, 0.7, 0.7],
                                graphics::ellipse::circle(item.0, item.1, 2.0),
                                context.transform,
                                graphics
                            );
                        }
                    }
                }
            });
        }

        // Mouse Left Button pressed, start of drawing
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            is_drawing = true;
            lines.push(Line::new());
        }

        // Mouse Left Button released, end of drawing
        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            is_drawing = false;

            if let Some(line) = lines.last_mut() {
                line.drawn = true;
            }
        }

        event.mouse_cursor(|x, y| {
            if is_drawing {
                if let Some(line) = lines.last_mut() {
                    line.push((x, y));
                }
            }
        });
    }
}
