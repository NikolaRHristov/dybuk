use term_painter::{Attr::*, Color::*, ToStyle};

use parse::{Message, Message::*};
use wrap::wrap_msg;

impl Message {
	pub fn print(self) {
		match self {
			Header(ref file, ref line) => {
				println!("+---- {} : {} ----+", Blue.bold().paint(file), Blue.paint(line))
			}
			Warning(warn) => println!(
				"      =====>  {}{}",
				Yellow.bold().paint("warning: "),
				Bold.paint(&wrap_msg(warn, 9))
			),
			Note(note) => println!(
				"      =====>  {}{}",
				Green.bold().paint("note: "),
				Bold.paint(&wrap_msg(note, 6))
			),
			Error(err) => println!(
				"      =====>  {}{}",
				Red.bold().paint("error: "),
				Bold.paint(&wrap_msg(err, 7))
			),
			Help(err) => println!(
				"      =====>  {}{}",
				Green.bold().paint("help: "),
				Bold.paint(&wrap_msg(err, 6))
			),
			FollowUp(msg) => println!("           >  {}", Bold.paint(msg)),
			Source(line, code) => println!(" {}  {}", Magenta.paint(format!("{} |>", line)), code),
			Etc => println!(" {}", Magenta.paint("...")),
			Marker(ref mrk) => println!("{}", Yellow.paint(mrk)),
			NewLine => println!("\n"),
			Wat => println!("Dafuq?"),
			Aborting => println!("\n{}", Red.paint("Aborting due to previous errors")),
		}
	}
}
