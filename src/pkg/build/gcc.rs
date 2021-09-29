use std::{
	path::Path,
	process::Command
};
use anyhow::bail;

pub(crate) fn try_compile(main_file: &Path, out_path: &Path) -> anyhow::Result<()> {
	// main_file should exist.

	// Compile main.cpp to main.dll in the package.
	let status = Command::new("gcc")
		.args( [&main_file.display().to_string(), "-o", &out_path.display().to_string(), "-shared"] )
		.status()?;

	if !status.success() {
		bail!("Command failed with code: {}",  status.code().unwrap_or(-1) );
	}

	Ok(())
}