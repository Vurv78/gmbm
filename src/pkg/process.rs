use super::Package;
use anyhow::bail;
use std::path::PathBuf;

#[derive(Debug)]
pub enum VerifyError {
	NotBuilt, // Not built yet. Need to call self.build()
	NoEntry, // No entrypoint (gmod13_open) was found
	NoExit, // No exitpoint (gmod13_close) was found

	Pe(pelite::Error) // Pelite error when trying to make PeFile.
}

impl std::fmt::Display for VerifyError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			VerifyError::NotBuilt => "Not built yet",
			VerifyError::NoEntry => "No entrypoint (gmod13_open) was found",
			VerifyError::NoExit => "No exitpoint (gmod13_close) was found",
			VerifyError::Pe(_) => "Pelite error"
		})
	}
}

pub type VerifyResult = Result<(), VerifyError>;

const GMOD_DLLOPEN: &str = "gmod13_open";
const GMOD_DLLCLOSE: &str = "gmod13_close";

pub enum BuildTarget {
	MSBuild(PathBuf),
	Cargo,
	CMake,
	Gcc(PathBuf),
	Premake5,
	NotFound
}

impl<'a> Package<'a> {
	pub fn verify(&self) -> VerifyResult {
		if let Some(ref fm) = self.filemap {
			let pe = pelite::PeFile::from_bytes( fm.as_ref() ).map_err(VerifyError::Pe)?;

			if pe.get_export_by_name(GMOD_DLLOPEN).is_err() {
				return Err( VerifyError::NoEntry );
			}

			if pe.get_export_by_name(GMOD_DLLCLOSE).is_err() {
				return Err( VerifyError::NoExit );
			}

			Ok(())
		} else {
			Err( VerifyError::NotBuilt )
		}
	}

	// Tries to find what to compile the package with.
	pub fn identify_target(&self) -> anyhow::Result<BuildTarget> {
		if !self.cache.exists() {
			bail!("Cache does not exist")
		}

		{
			let cargo_toml = self.repo.join("Cargo.toml");
			if cargo_toml.exists() {
				return Ok(BuildTarget::Cargo);
			}
		}

		{
			let cmakelists = self.repo.join("CMakeLists.txt");
			if cmakelists.exists() {
				return Ok(BuildTarget::CMake);
			}
		}

		{
			let pm = self.repo.join("premake5.lua");
			if pm.exists() {
				return Ok(BuildTarget::Premake5);
			}
		}

		let mut gcc = None;
		{
			for file in std::fs::read_dir(&self.repo)? {
				let file = file?;
				let path = file.path();
				if let Some(ext) = path.extension() {
					if ext == "sln" {
						return Ok(BuildTarget::MSBuild(path));
					} else if ext == "cpp" {
						gcc = Some(path)
					}
				}
			}
		}

		if let Some(main_cpp) = gcc {
			return Ok(BuildTarget::Gcc(main_cpp));
		}

		Ok(BuildTarget::NotFound)
	}
}