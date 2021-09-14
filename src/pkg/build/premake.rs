use super::BuildError;
use std::{
	path::PathBuf,
	process::{ExitStatus, Command}
};

pub(crate) fn try_compile(_cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> Result<ExitStatus, BuildError> {
	// Compile main.cpp to main.dll in the package.
	let status = Command::new("premake5")
		.args( ["vs2019"] )
		.status()
		.map_err(|x| BuildError::Io(x))?;

	Ok(status)
}