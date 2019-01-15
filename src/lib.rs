use graphics::math::{self, Vec2d, Matrix2d};

pub mod color;
pub mod poly;
pub mod storage;

#[derive(Copy,Clone)]
enum UndoStatus {
    InSync,
    At(usize),
}

pub struct Pizarra {
    offset: Vec2d,
    offset_t: Option<Matrix2d>,
    inv_offset: Option<Matrix2d>,
    dimentions: Vec2d,
    undo_status: UndoStatus,
    pub ctrl_on: bool,
    pub shift_on: bool,
}

impl Pizarra {
    pub fn new(dimentions: Vec2d) -> Pizarra {
        Pizarra {
            dimentions,
            offset: math::mul_scalar(dimentions, 0.5),
            inv_offset: None,
            offset_t: None,
            ctrl_on: false,
            shift_on: false,
            undo_status: UndoStatus::InSync,
        }
    }

    pub fn get_dimentions(&self) -> Vec2d {
        self.dimentions
    }

    pub fn get_inv_offset(&mut self) -> Matrix2d {
        match self.inv_offset {
            Some(inv) => inv,
            None => {
                let val = math::translate(math::mul_scalar(self.offset, -1.0));
                self.inv_offset = Some(val);

                val
            },
        }
    }

    pub fn get_offset_t(&mut self) -> Matrix2d {
        match self.offset_t {
            Some(t) => t,
            None => {
                let val = math::translate(self.offset);
                self.offset_t = Some(val);

                val
            }
        }
    }

    pub fn delta_offset(&mut self, delta: Vec2d) {
        self.offset = math::add(self.offset, delta);
        self.offset_t = None;
        self.inv_offset = None;
    }

    pub fn resize(&mut self, new_size: Vec2d) {
        let delta = math::mul_scalar(math::add(
            math::mul_scalar(self.dimentions, -1.0),
            new_size
        ), 0.5);

        self.delta_offset(delta);
        self.dimentions = new_size;
    }

    pub fn undo(&mut self) {
    }
}
