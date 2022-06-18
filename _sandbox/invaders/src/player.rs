use crate::NUM_COLS;
use crate::NUM_ROWS;
use crate::frame::{Drawable, Frame};

pub struct Player {
    // defines the properties of Player
    x: usize,
    y: usize,
}

 impl Player {
    pub fn new() -> Self {
        Self {
            // defines the (starting) location of the player.
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
        }
    }
    pub fn move_left(&mut self) {
        // move left
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        // move right
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
 }

 impl Drawable for Player {
    // draw the player out unto the frame (w a the char A)
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"
    }
 }