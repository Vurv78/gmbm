use std::{
	path::PathBuf,
	process::Command
};
use anyhow::bail;

pub(crate) fn try_compile(_cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> anyhow::Result<()> {
	// Compile main.cpp to main.dll in the package.
	let status = Command::new("premake5")
		.args( ["vs2019"] )
		.status()?;

	if !status.success() {
		bail!("Command failed with code: {}",  status.code().unwrap_or(-1) );
	}

	todo!();

	Ok(())
}