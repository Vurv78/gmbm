use super::prelude::*;

// premake5 support

pub(crate) fn try_compile(
	cache_dir: &Path,
	repo_dir: &Path,
	out_path: &Path,
) -> Result<(), TryCompileError> {
	// Compile main.cpp to main.dll in the package.
	let premake_master = crate::util::current_exe_dir()?
		.join("dat")
		.join("premake5.lua");

	let build_dir = cache_dir.join("premake");
	std::fs::create_dir_all(&build_dir)?;

	let status = Command::new("premake5")
		.env("PROJ_PATH", repo_dir.join("premake5.lua"))
		.env("BUILD_PATH", &build_dir)
		.args(["vs2019", &format!("--file={}", premake_master.display())])
		.status()?;

	if !status.success() {
		return Err(TryCompileError::CommandError(status.code().unwrap_or(-1)));
	}

	super::msbuild::try_compile(&build_dir, &build_dir, out_path)?;

	Ok(())
}
