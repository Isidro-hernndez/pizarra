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

use pizarra::Vec2D;
use pizarra::color::Color;

struct Line {
    points: Vec<Vec2D>,
    drawn: bool,
}

impl Line {
    fn new() -> Line {
        Line {
            points: Vec::with_capacity(1000),
            drawn: false,
        }
    }

    fn push(&mut self, val: Vec2D) {
        self.points.push(val);
    }
}

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: AppWindow = WindowSettings::new("Pizarra", [600, 600])
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let ref mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut is_drawing = false;
    let mut lines: Vec<Line> = Vec::new();
    let thickness = 1.0;

    // Colors
    let bgcolor = Color::black().to_a();
    let guidecolor = Color::gray().to_a();
    let drawcolor = Color::green().to_a();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                graphics::clear(bgcolor, graphics);

                graphics::line(
                    guidecolor,
                    thickness,
                    [10.0, 10.0, 40.0, 10.0],
                    context.transform,
                    graphics
                );

                for line in lines.iter() {
                    for ([x1, y1], [x2, y2]) in line.points.iter().zip(line.points.iter().skip(1)) {
                        graphics::line(
                            drawcolor,
                            thickness,
                            [*x1, *y1, *x2, *y2],
                            context.transform,
                            graphics
                        );
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
                    line.push([x, y]);
                }
            }
        });
    }
}
