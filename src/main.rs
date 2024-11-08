#![warn(clippy::all, clippy::pedantic)]

use rdev::{grab, Event, EventType, Key};
use std::env;
use std::fs;

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;
    path.push(screens_dir);

    fs::create_dir_all(path)?;

    if let Err(error) = grab(|e| callback(e, &screens_dir)) {
        println!("Error: {error:?}");
    }

    Ok(())
}

fn callback(event: Event, screens_dir: &String) {
    match event.event_type {
        EventType::KeyPress(Key::PrintScreen) => None,
        _ => Some(event),
    }
}
