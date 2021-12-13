use super::prelude::*;

pub(crate) fn try_compile(
	cache_dir: &Path,
	repo_dir: &Path,
	out_path: &Path,
) -> Result<(), TryCompileError> {
	let build_dir = cache_dir.join("cmake");

	let status = Command::new("cmake")
		.current_dir(repo_dir)
		.args([".", "-B..\\cmake"])
		.status()?;

	if !status.success() {
		return Err(TryCompileError::CommandError(status.code().unwrap_or(-1)));
	}

	super::msbuild::try_compile(&build_dir, &build_dir, out_path)?;

	Ok(())
}
