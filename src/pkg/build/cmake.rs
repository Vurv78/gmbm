use std::{
	path::PathBuf,
	process::Command
};
use anyhow::bail;

pub(crate) fn try_compile(cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> anyhow::Result<()> {
	let build_dir = cache_dir.join("cmake");

	let status = Command::new("cmake")
		.current_dir(repo_dir)
		.args( [ ".", "-B..\\cmake"] )
		.status()?;

	if !status.success() {
		bail!("Command failed with code: {}",  status.code().unwrap_or(-1) );
	}

	super::msbuild::try_compile(&build_dir, &build_dir, out_path)?;

	Ok(())
}