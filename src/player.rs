use crate::{
    frame::{Drawable, Frame},
    NUM_COLUMNS, NUM_ROWS,
};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLUMNS / 2,
            y: NUM_ROWS - 1,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLUMNS {
            self.x += 1
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"
    }
}
