use super::Package;

#[derive(Debug, thiserror::Error)]
pub enum TryCompileError {
	#[error("Failed to run compile command with code {0}")]
	CommandError(i32),
	#[error("Failed to traverse directory: {0}")]
	IOError(#[from] std::io::Error),
	#[error("Nothing was emitted by compilation. Is this a valid binary module?")]
	NoEmit,

	#[error("Couldn't find MSBuild, do you have Visual Studio installed?")]
	NoMSBuild,
}

pub(crate) mod prelude {
	pub(crate) use super::TryCompileError;
	pub(crate) use std::{path::Path, process::Command};
}

mod cargo;
mod cmake;
mod gcc;
mod msbuild;
mod premake;

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
	#[error("Failed to find directory of cloned repository, clone this first!")]
	RepoDirNotFound,

	#[error("Error finding build target: `{0}`")]
	BuildTargetError(#[from] crate::pkg::process::BuildTargetError),

	#[error("Error during compilation: `{0}`")]
	TryCompileError(#[from] TryCompileError),

	#[error("Pelite open error: `{0}`")]
	IOError(#[from] std::io::Error),
}

impl<'a> Package<'a> {
	pub fn build(&mut self) -> Result<(), BuildError> {
		let cache_dir = &self.cache;
		let repo_dir = &self.cache.join("repo");

		if !repo_dir.exists() {
			return Err(BuildError::RepoDirNotFound);
		}

		let out_path = cache_dir.join("main.dll");

		use crate::pkg::process::BuildTarget::*;
		match self.identify_target()? {
			Cargo => cargo::try_compile(cache_dir, repo_dir, &out_path)?,
			MSBuild(path) => msbuild::try_compile(cache_dir, &path, &out_path)?,
			CMake => cmake::try_compile(cache_dir, repo_dir, &out_path)?,
			Gcc(path) => gcc::try_compile(&path, &out_path)?,
			Premake5 => premake::try_compile(cache_dir, repo_dir, &out_path)?,
		}

		self.filemap = Some(pelite::FileMap::open(&out_path)?);
		Ok(())
	}
}
