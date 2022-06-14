use crate::{NUM_ROWS, NUM_COLS};

// Vector of Vectors of borrowed static str slices.
pub type Frame = Vec<Vec<&'static str>>; // this is type alias
// The term "Frame" confuses me at first, but looking at the code now ..
// just think of it as a grid
// since it is a vector of vectors
// substitute vector w array and we'll get 
// array or arrays aka a grid

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS); // create a Vector with a capacity of NUM_COLS 
    for _ in 0..NUM_COLS { // for each _ in cols 
        let mut col = Vec::with_capacity(NUM_ROWS); // create a Vector with a capacity of NUM_ROWS 
        for _ in 0..NUM_ROWS {
            col.push(" "); // for each _ in cols, create a " " - blank frame
            // if i == 0 {
            //     col.push("0");
            // } else if i == 1 {
            //     col.push("1");
            // } else {
            //     col.push("X");
            // }
        }
        cols.push(col)  // push col (made up of _) into cols
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame); // take an immutable reference to yourself and a mutable reference to a frame and then draw itself into the frame(?)
}