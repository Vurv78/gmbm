use std::path::PathBuf;
use anyhow::{anyhow, bail};

pub(crate) fn try_compile(cache_dir: &PathBuf, repo_dir: &PathBuf, out_path: &PathBuf) -> anyhow::Result<()> {
	// Todo: target should probably not be hardcoded
	let msbuild = cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "msbuild")
		.ok_or( anyhow!("Couldn't find msbuild, make sure you have Visual Studio/MSBuild installed!") )?;

	let build_dir = cache_dir.join("msbuild");
	std::fs::create_dir_all(&build_dir)?;

	let status = msbuild
		.to_command()
		.current_dir(repo_dir)
		.args( [".", "-noLogo", "-m:5", "-verbosity:quiet", &format!("-p:OutDir={}", build_dir.display()), "-p:TargetName=main", "-p:Configuration=Release"] )
		.status()?;

	if status.success() {
		std::fs::copy( build_dir.join("main.dll"), out_path )?;
	} else {
		// Should be normalized.
		bail!("Command failed with code: {}",  status.code().unwrap_or(-1) );
	}

	Ok(())
}