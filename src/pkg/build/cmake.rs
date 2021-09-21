use std::{
	path::PathBuf,
	process::Command
};
use anyhow::bail;

pub(crate) fn try_compile(cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> anyhow::Result<()> {
	let build_dir = cache_dir.join("cmake");

	cmake::Config::new(".")
		.out_dir(&build_dir)
		.build();

	super::msbuild::try_compile(&build_dir, &build_dir, out_path)?;

	Ok(())
}