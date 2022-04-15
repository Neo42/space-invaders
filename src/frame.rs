use crate::{NUM_COLUMNS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut columns = Vec::with_capacity(NUM_COLUMNS);
    for _ in 0..NUM_COLUMNS {
        let mut column = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            column.push(" ")
        }
        columns.push(column)
    }
    columns
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
