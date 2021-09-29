use std::fs::{self, File};
use std::io::Write;

use super::{Package, PackageInfo};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitError {
	#[error("IO Error: `{0}`")]
	Io(std::io::Error),
	#[error("Failed to serialize TOML. `{0}`")]
	Serde(toml::ser::Error),
}

impl<'a> Package<'a> {
	pub fn init(&self) -> Result<(), InitError> {
		fs::create_dir_all(&self.cache).map_err(InitError::Io)?;

		let mut f = File::create(self.cache.join("pkg.toml")).map_err(InitError::Io)?;

		let info = PackageInfo {
			name: self.name,
			repo_url: self.repo_url.as_str()
		};

		let toml_raw = toml::to_string(&info).map_err(InitError::Serde)?;
		f.write_all(toml_raw.as_bytes()).map_err(InitError::Io)?;

		Ok(())
	}
}