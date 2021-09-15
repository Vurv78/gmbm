use std::path::PathBuf;

use std::process::Command;
use anyhow::bail;

pub(crate) fn try_compile(cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> anyhow::Result<()> {
	let build_dir = cache_dir.join("cargo_target");
	let release_dir = build_dir.join("release");

	std::env::set_var("CARGO_TARGET_DIR", build_dir.display().to_string());

	let status = Command::new("cargo")
		.current_dir(repo_dir)
		.args( [ "build", "--release"] )
		.status()?;

	if !status.success() {
		bail!("Failed with code: {}", status.code().unwrap_or(-1));
	}

	for file in std::fs::read_dir(release_dir)? {
		let file = file?;
		let p = file.path();
		if let Some(e) = p.extension() {
			if e == "dll" {
				std::fs::copy(&p, out_path)?;
				return Ok(());
			}
		}
	}

	bail!("Build didn't emit a dll file. Is this a proper binary module?");
}