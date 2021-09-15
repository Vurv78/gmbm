use super::Package;
use anyhow::bail;

mod gcc;
mod msbuild;
mod cmake;
mod premake;
mod cargo;

#[derive(Debug)]
pub enum BuildError {
	AlreadyBuilt,
	Io(std::io::Error),
	NoMSBuild,
	Missing,
	CommandFailure(i32)
}

impl std::fmt::Display for BuildError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			BuildError::AlreadyBuilt => "Already Built".to_owned(),
			BuildError::Io(err) => format!("IO Error: {}", err),
			BuildError::NoMSBuild => "Couldn't find msbuild, make sure you have visual studio/msbuild installed!".to_owned(),
			BuildError::Missing => "Package doesn't exist, clone it first.".to_owned(),
			BuildError::CommandFailure(err) => format!("Command failed with code: {}", err),
		})
	}
}

impl<'a> Package<'a> {
	pub fn build(&mut self) -> anyhow::Result<()>{
		let cache_dir = &self.cache;
		let repo_dir = &self.cache.join("repo");

		if !repo_dir.exists() {
			bail!("Repo directory doesn't exist. Not cloned yet?")
		}

		let out_path = cache_dir
			.join("main.dll");

		use crate::pkg::process::BuildTarget::*;
		match self.identify_target() {
			Ok(x) => match x {
				Cargo => {
					if let Err(why) = cargo::try_compile(cache_dir, repo_dir, &out_path) {
						bail!("{}", why)
					}
				},
				MSBuild => {
					if let Err(why) = msbuild::try_compile(cache_dir, repo_dir, &out_path) {
						bail!("{}", why)
					}
				},
				CMake => {
					if let Err(why) = cmake::try_compile(cache_dir, repo_dir, &out_path) {
						bail!("{}", why)
					}
				}
				NotFound => bail!("Unknown compiler")
			}
			Err(why) => bail!(why)
		}

		self.filemap = Some( pelite::FileMap::open(&out_path)? );
		Ok(())
	}
}