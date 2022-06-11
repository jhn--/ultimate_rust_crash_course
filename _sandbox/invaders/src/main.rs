use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("pew", "./sounds/pew.wav");
    audio.add("startup", "./sounds/startup.wav");
    audio.add("win", "./sounds/win.wav");
    audio.play("startup");

    audio.wait();
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
