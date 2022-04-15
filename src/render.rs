use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force_render: bool) {
    if force_render {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, column) in current_frame.iter().enumerate() {
        for (y, character) in column.iter().enumerate() {
            if *character != last_frame[x][y] || force_render {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *character)
            }
        }
    }
    stdout.flush().unwrap()
}
