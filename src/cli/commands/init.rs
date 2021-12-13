use super::prelude::*;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn init(_: &ArgMatches) -> Result<(), CommandError> {
	let pkg_dir = crate::util::current_exe_dir()?;

	let package_dir = pkg_dir.join("cache"); // Package source install directories, where they're built and then installed to your gmod dir.
	let lib_dir = pkg_dir.join("libs"); // Library install directory, where stuff like GARRYSMOD_COMMON is installed and kept for all packages to use.
	let dat_dir = pkg_dir.join("dat"); // Generic data files.

	fs::create_dir_all(&package_dir)?;
	fs::create_dir_all(&lib_dir)?;
	fs::create_dir_all(&dat_dir)?;

	let mut handle = File::create(dat_dir.join("premake5.lua"))?;
	handle.write_all(include_bytes!("../../premake5.lua"))?;

	Ok(())
}
