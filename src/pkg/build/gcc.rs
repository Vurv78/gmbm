use super::BuildError;
use std::{
	path::PathBuf,
	process::{ExitStatus, Command}
};

pub(crate) fn try_compile(_cache_dir: &PathBuf, main_path: &PathBuf, out_path: &PathBuf) -> Result<ExitStatus, BuildError> {
	// Compile main.cpp to main.dll in the package.
	let status = Command::new("gcc")
		.args( [&main_path.display().to_string(), "-o", &out_path.display().to_string(), "-shared"] )
		.status()
		.map_err(|x| BuildError::Io(x))?;

	Ok(status)
}