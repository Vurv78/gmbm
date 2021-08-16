use super::BuildError;
use std::path::PathBuf;

pub(crate) fn try_compile(cache_dir: &PathBuf, main_path: &PathBuf, out_path: &PathBuf) -> Result<(), BuildError> {
	// Todo: target should probably not be hardcoded
	let msbuild = cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "msbuild")
		.ok_or(BuildError::NoMSBuild)?;

	let main_dir = main_path.parent().unwrap().display().to_string();

	let build_dir = cache_dir.join("msbuild");
	std::fs::create_dir_all(&build_dir).map_err( |x| BuildError::Io(x) )?;

	let code = msbuild
		.to_command()
		.args( [&main_dir, "-noLogo", "-m:5", "-verbosity:quiet", &format!("-p:OutDir={}", build_dir.display()), "-p:TargetName=main", "-p:Configuration=Release" ] )
		.status()
		.map_err(|x| BuildError::Io(x))?;

	if code.success() {
		std::fs::copy( build_dir.join("main.dll"), out_path )
			.map_err(|x| BuildError::Io(x))?;
	} else {
		return Err( BuildError::CommandFailure( code.code().unwrap_or(-1) ) );
	}

	Ok(())
}