use std::error::Error;
// use rusty_audio::Audio; /* sounds */
use std::io;
use std::sync::mpsc;
use std::thread;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Show, Hide};
use crossterm::event::{self, Event, KeyCode};

use std::time::Duration;
use std::time::Instant;

use invaders::frame;
use invaders::frame::{new_frame, Drawable};
use invaders::render;

use invaders::player::Player;
use invaders::invaders::Invaders;

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

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop
    let mut player = Player::new();
    
    let mut instant = Instant::now();

    let mut invaders = Invaders::new();

    'gameloop: loop { // label the loop w the name gameloop
        //Per-frame initialization
        let delta = instant.elapsed(); // Returns the amount of time elapsed since this instant was created.
        instant = Instant::now(); // refresh the value of instant.

        let mut curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            // poll for input events with a default duration of 0
            // once input events occur, don't wait, respond immediately
            if let Event::Key(key_event) = event::read()? {
                // what sort of events are we interested in?
                match key_event.code {
                    // keyboard events
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        // bind keyboard keys to the shooting action
                        if player.shoot() {
                            // player.shoot returns a boolean
                            // it returns a boolean because 
                            // we want to play the "pew" sound
                            // the return boolean has no other role

                            // audio.play("pew");
                        }
                    },
                    KeyCode::Left | KeyCode::Char('a') => {
                        // bind keyboard keys to player's movement to left
                        player.move_left();
                    },
                    KeyCode::Right | KeyCode::Char('d') => {
                        // bind keyboard keys to player's movement to right
                        player.move_right();
                    },
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

        // Updates
        // updates here basically updates .. afaics, 1 thing:
        // - graphics, ie, movement of the invaders, shots, player
        player.update(delta); // pass the delta into player.update, which will then update the status of the shots
        if invaders.update(delta) {
            // audio.play("move"); 
        }
        if player.detect_hits(&mut invaders) {
            // audio.play("explode");
        }

        // Draw & render

        // player.draw(&mut curr_frame); // draw the Player
        // invaders.draw(&mut curr_frame); // draw the Invaders
        // player.draw and invaders.draw works but Nathan wants to use Generics so lets go w it (see below)
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // win or lose
        if invaders.all_killed() {
            // audio.play("win");
            println!("you win");
            thread::sleep(Duration::from_millis(5000));
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            // audio.play("lose");
            println!("you lose");
            thread::sleep(Duration::from_millis(5000));
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

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
