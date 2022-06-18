// mod - modules
// pub - public (accessible externally)
pub mod frame;
pub mod render; // Add a conceptual frame out to the actual terminal
pub mod player; // Add player module
pub mod shot; // Add shot module

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;