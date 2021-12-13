use super::prelude::*;

pub(crate) fn try_compile(
	cache_dir: &Path,
	repo_dir: &Path,
	out_path: &Path,
) -> Result<(), TryCompileError> {
	let build_dir = cache_dir.join("cargo_target");
	let release_dir = build_dir.join("release");

	std::env::set_var("CARGO_TARGET_DIR", build_dir.display().to_string());

	let status = Command::new("cargo")
		.current_dir(repo_dir)
		.args(["build", "--release", "--quiet"])
		.status()?;

	if !status.success() {
		return Err(TryCompileError::CommandError(status.code().unwrap_or(-1)));
	}

	for file in std::fs::read_dir(release_dir)?.flatten() {
		let p = file.path();
		if let Some(e) = p.extension() {
			if e == "dll" {
				std::fs::copy(&p, out_path)?;
				return Ok(());
			}
		}
	}

	Err(TryCompileError::NoEmit)
}
