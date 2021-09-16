use super::Package;
use anyhow::bail;

mod gcc;
mod msbuild;
mod cmake;
mod cargo;
mod premake;

impl<'a> Package<'a> {
	pub fn build(&mut self) -> anyhow::Result<()> {
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
				Cargo => cargo::try_compile(cache_dir, repo_dir, &out_path)?,
				MSBuild => msbuild::try_compile(cache_dir, repo_dir, &out_path)?,
				CMake => cmake::try_compile(cache_dir, repo_dir, &out_path)?,
				GCC(path) => gcc::try_compile(&path, &out_path)?,
				Premake5 => premake::try_compile(cache_dir, repo_dir, &out_path)?,
				NotFound => bail!("Unknown compiler")
			},
			Err(why) => bail!(why)
		}

		self.filemap = Some( pelite::FileMap::open(&out_path)? );
		Ok(())
	}
}