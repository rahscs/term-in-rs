extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
	// Get the standard input stream.
	let stdin = stdin();
	// Get the standard output stream and go to raw mode.
	let mut stdout = stdout().into_raw_mode().unwrap();

	write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
		// Clear screen.
		termion::clear::All,
		// Move cursor to first row, first column.
		termion::cursor::Goto(1, 1),
		// Hide cursor.
		termion::cursor::Hide).unwrap();
	// Update terminal screen.
	stdout.flush().unwrap();

	// Main loop
	for c in stdin.keys() {
		// Clear first line.
		write!(stdout, "{}{}", termion::cursor::Goto(1, 1),
			termion::clear::CurrentLine).unwrap();

		// Print any ke that's pressed.
		match c.unwrap() {
			Key::Char('q') => break, // Quit
			Key::Char(c)   => println!("{}", c),
			Key::Alt(c)    => println!("Alt-{}", c),
			Key::Ctrl(c)   => println!("Ctrl-{}", c),
			Key::Left      => println!("<left>"),
			Key::Right     => println!("<right>"),
			Key::Up        => println!("<up>"),
			Key::Down      => println!("<down>"),
			_              => println!("Other"),
		}

		// Update terminal screen.
		stdout.flush().unwrap();
	}

	// Unhide Cursor
	write!(stdout, "{}", termion::cursor::Show).unwrap();
}
