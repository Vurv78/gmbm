use super::prelude::*;

pub(crate) fn try_compile(
	cache_dir: &Path,
	sln_path: &Path,
	out_path: &Path,
) -> Result<(), TryCompileError> {
	// Todo: target should probably not be hardcoded
	let msbuild = cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "msbuild")
		.ok_or(TryCompileError::NoMSBuild)?;

	let build_dir = cache_dir.join("msbuild");
	std::fs::create_dir_all(&build_dir)?;

	let mut cmd = msbuild.to_command();
	cmd.arg(sln_path.as_os_str());

	cmd.args([
		"-noLogo",
		"-m:5",
		"-verbosity:quiet",
		&format!("-p:OutDir={}", build_dir.display()),
		"-p:TargetName=main",
		"-p:Configuration=Release",
		"-p:Platform=x64",
	]);

	let status = cmd.status()?;

	if status.success() {
		std::fs::copy(build_dir.join("main.dll"), out_path)?;
	} else {
		// Should be normalized.
		return Err(TryCompileError::CommandError(status.code().unwrap_or(-1)));
	}

	Ok(())
}
