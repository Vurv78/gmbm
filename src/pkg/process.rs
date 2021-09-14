use super::Package;
use anyhow::bail;

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

pub enum CompilerTarget {
	MSBuild,
	Cargo,
	NotFound
}

impl<'a> Package<'a> {
	pub fn verify(&self) -> VerifyResult {
		if let Some(ref fm) = self.filemap {
			let pe = pelite::PeFile::from_bytes( fm.as_ref() ).map_err(VerifyError::Pe)?;

			if let Err(_) = pe.get_export_by_name(GMOD_DLLOPEN) {
				return Err( VerifyError::NoEntry );
			}

			if let Err(_) = pe.get_export_by_name(GMOD_DLLCLOSE) {
				return Err( VerifyError::NoExit );
			}

			Ok(())
		} else {
			Err( VerifyError::NotBuilt )
		}
	}

	// Tries to find what to compile the package with.
	pub fn identify_compiler(&self) -> anyhow::Result<CompilerTarget> {
		if !self.cache.exists() {
			bail!("Cache does not exist")
		}

		{
			let cargo_toml = self.cache.join("Cargo.toml");
			if cargo_toml.exists() {
				return Ok(CompilerTarget::Cargo);
			}
		}

		{
			for file in std::fs::read_dir(&self.repo)? {
				let file = file?;
				let path = file.path();
				if let Some(ext) = path.extension() {
					if ext == "sln" {
						return Ok(CompilerTarget::MSBuild);
					}
				}
			}
		}

		Ok(CompilerTarget::NotFound)
	}
}