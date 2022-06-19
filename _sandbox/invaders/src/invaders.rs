use rusty_time::timer::Timer;
use std::time::Duration;
 use std::cmp::max;
 use crate::frame::Drawable;
 use crate::frame::Frame;

use crate::{NUM_COLS, NUM_ROWS};

pub struct Invader {
    // Invader will just have a position
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    // Invaders contain Invader
    // more importantly -
    // they have a move_timer which determines the speed of their movement
    // and a direction
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    // implementing the Invaders struct
    pub fn new() -> Self {
        let mut army = Vec::new(); // define the Vec
        for x in 0..NUM_COLS { 
            // populating the Vec in the following format -
            // + + + + + + + + + + + + + + + + + +

            // + + + + + + + + + + + + + + + + + +

            // + + + + + + + + + + + + + + + + + +

            // + + + + + + + + + + + + + + + + + +
            for y in 0..NUM_ROWS{
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                    // && (x == 2) // for testing win scenario
                    {
                        army.push(Invader { x, y });
                    }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            // move_timer: Timer::from_millis(250), // for testing win scenario
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        // update deals with the movement of Invaders
        // left, down, right, down - repeat
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 { // move left
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else { // move right
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                // will out of bounds as we didn't define what happens when invader.y exceeds NUM_ROWS
                // ^^^ is resolved w reached_bottom()
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                // move to the side (either left or right)
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                    // this line ^^^, by casting invader.x as an i32, which can go into negative
                    // we satisfy the need to check if the direction is positive(left) or negative(right)
                }
            }
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        // accepts the x and y from shots and compare w the x and y from all invader in invaders
        if let Some(idx) = self.army.iter().position(|invader| (invader.x == x) && (invader.y == y)) {
            // Some() means any (or none)
            // remove the idx (index) from the Vec
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            // change the image every .5 sec
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32() 
                / self.move_timer.duration.as_secs_f32()) > 0.5 {
                    "X"
                } else {
                    "+"
                };
        }
    }
}