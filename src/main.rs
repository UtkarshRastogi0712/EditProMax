use crossterm::terminal;
use std::io;
use std::io::Read;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not turn off raw mode");
    }
}

fn main() {
    let _cleanup = Cleanup;
    terminal::enable_raw_mode().expect("Unable to enter raw mode");
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
        let character = buf[0] as char;
        if character.is_control() {
            println!("{}", character as u8)
        } else {
            println!("{}", character)
        }
    }
}
