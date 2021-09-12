use std::error::Error;

use clap::ArgMatches;
use super::Package;

pub fn init(x: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
	let pkg_dir = std::env::current_dir()?;

	let package_dir = pkg_dir.join("cache"); // Package source install directories, where they're built and then installed to your gmod dir.
	let lib_dir = pkg_dir.join("libs"); // Library install directory, where stuff like GARRYSMOD_COMMON is installed and kept for all packages to use.

	std::fs::create_dir_all(package_dir)?;
	std::fs::create_dir_all(lib_dir)?;

	Ok(())
}