/*
* Put information about the project here
*/

// common stuff to import
use indoc::indoc;
use text_io::read;

// locals
mod consts;
use consts::{LINE_SEPARATOR};

mod classes;
use classes::*;

mod sub;

pub fn main(args: Vec<String>) -> IDFC<()> {
	if args.len() < 2 { return Ok(show_help()); }

	let cmd = &args[1].to_lowercase();	// give the cmd its own binding
	let args = &args[2..];				// shadow first vec

	let cmd = cmd_replace_aliases(cmd);

	match cmd {
		"amogus"	=> {
			assert_argc(args, &[0]);
			sub::print();
		}

		_	=> show_help()
	}

	// do stuff
	Ok(())
}

pub fn show_help() {
	println!("{}\n{}{}\n", LINE_SEPARATOR, include_str!("help.txt"), LINE_SEPARATOR);
}

// Check argument len
pub fn assert_argc(args: &[String], lens: &[usize]) {
	let len = args.len();

	let mapped: Vec<String> = lens.iter().map(|&id| id.to_string()).collect();
	let joined = mapped.join("|");

	if !lens.contains(&len) {
		panic!("This subcommand requires {} arguments, but you only gave {}!", joined, len);
	}
}

fn cmd_replace_aliases<'a>(cmd: &'a String) -> &'a str {
	match cmd.as_str() {
//		"alias"	=> "somethingelse",
		_		=> &cmd
	}
}
