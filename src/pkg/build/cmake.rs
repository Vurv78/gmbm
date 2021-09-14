use super::BuildError;
use std::path::PathBuf;

use std::process::Command;

pub(crate) fn try_compile(cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> Result<(), BuildError> {
	let build_dir = cache_dir.join("cmake");

	let status = Command::new("cmake")
		.current_dir(repo_dir)
		.args( [ ".", "-B..\\cmake"] )
		.status()
		.map_err(|x| BuildError::Io(x))?;

	if !status.success() {
		return Err(BuildError::CommandFailure(status.code().unwrap_or(-1)));
	}

	super::msbuild::try_compile(&build_dir, &build_dir, out_path)?;

	Ok(())
}