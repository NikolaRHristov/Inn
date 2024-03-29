pub fn Fn(Option { Command, Entry, Pattern, Separator, .. }: Option) {
	Entry
		.into_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.for_each(|Entry| {
			let mut Command = Command::new(Command.get(0).expect("Cannot Command."))
				.args(&Command[1..])
				.current_dir(Entry)
				.stdout(Stdio::piped())
				.spawn()
				.expect("Cannot spawn.")
				.stdout
				.expect("Cannot stdout.");

			let mut Output = String::new();

			loop {
				let mut Buffer = [0; 512];
				let Byte = Command.read(&mut Buffer).expect("Cannot read.");

				if Byte == 0 {
					break;
				}

				Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
			}

			println!("{}", Output);
		})
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

use std::{
	io::Read,
	process::{Command, Stdio},
};
