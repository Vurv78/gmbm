mod pkg;

fn main() -> anyhow::Result<()> {
	use colored::*;

	if let Err(why) = pkg::cli::process() {
		println!( "Error when setting up cli. {}", why.to_string().red() );
	}

	Ok(())
}
