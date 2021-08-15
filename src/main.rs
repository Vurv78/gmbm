mod pkg;

#[macro_use] extern crate log;

fn main() -> anyhow::Result<()> {
	if let Err(why) = pkg::cli::process() {
		error!("Error when setting up cli. {}", why);
	}

	Ok(())
}
