use super::BuildError;
use std::{
	path::PathBuf,
	process::{ExitStatus, Command}
};

pub(crate) fn try_compile(cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> Result<ExitStatus, BuildError> {
	let main_cpp = repo_dir.join("main.cpp");
	if !main_cpp.exists() {
		return Err( BuildError::Missing );
	}

	// Compile main.cpp to main.dll in the package.
	let status = Command::new("gcc")
		.args( [&main_cpp.display().to_string(), "-o", &out_path.display().to_string(), "-shared"] )
		.status()
		.map_err(|x| BuildError::Io(x))?;

	Ok(status)
}