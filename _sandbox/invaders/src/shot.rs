use crate::frame::Drawable;
use crate::frame::Frame;
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer, // internal timer to keep track of the shot's movement
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x, 
            y,
            exploding: false,
            timer: Timer::from_millis(50), // laser will move 1 cell every 50ms.
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta); // make timer count down by delta amount
        if self.timer.ready && !self.exploding {
            // if timer is ready and shot is not exploding
            if self.y > 0 { // if shot has not reached the end of the screen
                self.y -= 1; // move the shot upwards
            }
            self.timer.reset(); // reset the timer
        }
    }
    pub fn explode(&mut self) {
        // when explode() is called
        self.exploding = true; // set exploding to be true
        self.timer = Timer::from_millis(250); // set the duration of the explosion to be 250ms
    }
    pub fn dead(&self) -> bool {
        // shot is dead
        // what does it mean? 
        // well, it means when the shot has expired, 
        // i.e. exploded and timer expired or reached the end of the screen
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    // now draw the shot
    fn draw(&self, frame: &mut Frame) {
        // if shot is exploding then set "*" at the location (x,y)
        // otherwise, shot is in motion, therefore "|"
        frame[self.x][self.y] = if self.exploding {
            "*"
        } else {
            "|"
        };
    }
}