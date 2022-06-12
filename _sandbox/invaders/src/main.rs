use std::error::Error;
// use rusty_audio::Audio; /* sounds */
use std::io;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Show, Hide};
use crossterm::event::{self, Event, KeyCode};

use std::time::Duration;

fn main() -> Result <(), Box<dyn Error>> {
    /* sounds */
    // let mut audio = Audio::new();
    // audio.add("explode", "./sounds/explode.wav");
    // audio.add("lose", "./sounds/lose.wav");
    // audio.add("move", "./sounds/move.wav");
    // audio.add("pew", "./sounds/pew.wav");
    // audio.add("startup", "./sounds/startup.wav");
    // audio.add("win", "./sounds/win.wav");
    // audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();

    // enable raw mode so we can get keyboard input as it occurs.
    // append a `?` operator - will crash the program if we have an error.
    terminal::enable_raw_mode()?;
    // enter alternate screen
    stdout.execute(EnterAlternateScreen)?; // enter alternate screen
    stdout.execute(Hide)?; // hide cursor

    // Game Loop
    'gameloop: loop { // label the loop w the name gameloop
        // Input
        while event::poll(Duration::default())? {
            // poll for input events with a default duration of 0
            // once input events occur, don't wait, respond immediately
            if let Event::Key(key_event) = event::read()? {
                // what sort of events are we interested in?
                match key_event.code {
                    // keyboard events
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // specifically Esc key and q key
                        // audio.play("lose");
                        break 'gameloop; // break out of the game loop
                    },
                    _ => {
                        // ignore everything else
                    }
                }
            }
        }
    }
    
    // Cleanup

    /* sounds */
    // audio.wait();
    
    stdout.execute(Show)?; // show cursor
    stdout.execute(LeaveAlternateScreen)?; // leave alternate screen
    terminal::disable_raw_mode()?; // leave raw mode
    
    Ok(())
}

/*
trying to use std::path::Path to list all wav files 
in ./sounds/ and automatically add them to audio.
Fail spectacularly, looking for answers, revisit when I have the answer
*/
// #![deny(elided_lifetimes_in_paths)] 
// #![warn(unused_variables)]

// use std::path::Path;
// use std::error::Error;
// use rusty_audio::Audio;

// const AUDIO_PATH: &str = "./sounds/";
// const WAV: &str = ".wav";

// fn main() -> Result <(), Box<dyn Error>> {
// // fn main() {
//     let audio_path = Path::new(AUDIO_PATH);
//     let mut audio = Audio::new();

//     for af in audio_path.read_dir().expect("") {
//         if let Ok(af) = af {
//             println!("{:?}", af.path());

//             let full_audio_path: String = af.path().display().to_string();
//             let audio_name: String = af.path().display().to_string()[AUDIO_PATH.len()..(af.path().display().to_string().len()-WAV.len())].to_string(); // produces the name, i.e. "test" from "./test.wave"
        
//             audio.add(&audio_name, &full_audio_path);
//         }
//     }
//     audio.play("startup");
    
//     audio.wait();
//     Ok(())
// }
