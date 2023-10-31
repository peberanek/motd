// Print a message of the day (motd) into stdout.
//
// Messages are selected at random from `~/.motd` (1 message per line).
// Blank lines and lines starting with `#` are ignored.

use home::home_dir;
use rand::seq::SliceRandom;
use std::{fs, path::PathBuf};

/// Return a random message of the day (motd) from `file`.
///
/// Blank lines and lines starting with `#` are ignored.
///
/// # Arguments
///
/// * `file` - A file containing messages of the day.
///
fn read_message(file: PathBuf) -> String {
    let contents = fs::read_to_string(file).expect("Should be able to read `~/.motd`.");
    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .collect();
    let mut rng = rand::thread_rng();

    lines.choose(&mut rng).unwrap().to_string()
}

fn main() {
    let input_file = home_dir()
        .expect("Should be able to get home directory.")
        .join(".motd");

    println!("{}", read_message(input_file))
}
