use std::io::{Stdout, Write};
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{ClearType, Clear};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // take a stdout as a mutable reference
    // render only what's changed - to do that we will needa  last frame which will be a 
    // reference to a Frame and a current Frame
    // we'll going to have to draw everything at least once, hence force it.
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (x, col) in curr_frame.iter().enumerate() {
        // for every x index of column vectors in our current frame,
        // we'll iterate through immutably and enumerate to get 
        // x index 
        for (y, s) in col.iter().enumerate() {
            // for everything y index and actual string character (s) 
            // iterate through immutably and enumerate to get 
            // y index
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                println!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}