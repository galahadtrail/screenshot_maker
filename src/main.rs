#![warn(clippy::all, clippy::pedantic)]

use std::env;
use std::fs;

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;
    path.push(screens_dir);

    fs::create_dir_all(path)?;

    Ok(())
}
