use graphics::math::Vec2d;

pub mod line;

pub use self::line::Line;

pub enum DrawCommand {
    Line{
        color: [f32; 4],
        thickness: f64,
        line: [f64; 4],
    },
}

pub trait Shape {
    fn handle(&mut self, val: Vec2d);
    // TODO review the posibility of sending an iterator to prevent the
    // allocation of a vector
    fn draw_commands(&self) -> Vec<DrawCommand>;
}
