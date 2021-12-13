mod cli;
mod pkg;
mod util;

fn main() {
	use colored::*;

	if let Err(why) = cli::process() {
		println!("Error when setting up cli. {}", why.to_string().red());
	}
}
