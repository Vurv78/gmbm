use super::Package;

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
}