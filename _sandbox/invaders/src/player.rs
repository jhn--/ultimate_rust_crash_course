use crate::NUM_COLS;
use crate::NUM_ROWS;
use crate::frame::{Drawable, Frame};
use crate::shot::Shot;

use std::time::Duration;

use crate::invaders::Invaders;

pub struct Player {
    // defines the properties of Player
    x: usize,
    y: usize,
    shots: Vec<Shot>, // have multiple shots 
}

 impl Player {
    pub fn new() -> Self {
        Self {
            // defines the (starting) location of the player.
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
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
    pub fn shoot(&mut self) -> bool { // player shooting the shot(s)
        // return a boolean for whether we have successfully shot (why?)
        // if the max number of shots are already on the screen
        // we might not actually successfully shoot.
        // so this vector keeps track of .. active shots?
        if self.shots.len() < 2 { 
            // if the number of active shots is lt 2
            // then "shoot"
            self.shots.push(Shot::new(self.x, self.y - 1));
            true // and return true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        // keep track of our shots
        for shot in self.shots.iter_mut() {
            // go through every shot in our shots
            shot.update(delta); // and update the timer
        }
        self.shots.retain(|shot| !shot.dead()); // retain the shots (which aren't dead) in vector
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        // this fn is basically called by main.rs only to ... play audio
        let mut hit_something = false; // we begin w nothing's been hit
        for shot in self.shots.iter_mut() {
            // we check through our shots
            if !shot.exploding {
                // if any of the shot is not exploding
                if invaders.kill_invader_at(shot.x, shot.y) {
                    // apply the location of the shot to the location of the invader in invaders
                    // and if we get a true
                    hit_something = true; // then we've hit an invader w our shot
                    shot.exploding; // and we set the shot.exploding to be true
                }
            }
        }
        hit_something // this will return to main.rs
    }
 }

 impl Drawable for Player {
    // draw the player out unto the frame (w a the char A)
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            // draw the shot(s) in the vector
            shot.draw(frame);
        }
    }
 }