pub(crate) mod build;
pub(crate) mod clone;
pub(crate) mod init;
pub(crate) mod process;

use std::{io::Read, path::PathBuf};
use url::Url;

pub struct Package<'a> {
	pub name: &'a str,
	pub repo_url: Url,
	pub mpath: PathBuf, // Current exe dir by default

	pub cache: PathBuf,
	pub repo: PathBuf,
	pub(crate) filemap: Option<pelite::FileMap>,
}

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct PackageInfo<'a> {
	name: &'a str,
	repo_url: &'a str,
}

#[derive(Debug, thiserror::Error)]
pub enum CreationError {
	#[error("Package already exists")]
	Exists,
	#[error("IO Error: `{0}`")]
	IO(std::io::Error),
	#[error("Error when initializing: `{0}`")]
	Init(init::InitError),
}

#[derive(Debug, thiserror::Error)]
pub enum PackageOpenError {
	#[error("Package does not exist")]
	DoesNotExist,
	#[error("Package is malformed. Missing pkg.toml")]
	Malformed,
	#[error("IO Error: `{0}`")]
	IO(#[from] std::io::Error),
	#[error("Could not parse pkg.toml: `{0}`")]
	TomlParse(#[from] toml::de::Error),
	#[error("Could not parse URL `{0}`")]
	UrlParse(#[from] url::ParseError),
}

impl<'a> Package<'a> {
	pub fn create(name: &'a str, repo_url: Url, mpath: PathBuf) -> Result<Self, CreationError> {
		let cache = mpath.join("cache").join(name);

		if cache.exists() {
			return Err(CreationError::Exists);
		}

		let repo_dir = cache.join("repo");

		if let Err(why) = std::fs::create_dir_all(&cache) {
			return Err(CreationError::IO(why));
		}

		let this = Self {
			name,
			repo_url,
			mpath,

			cache,
			repo: repo_dir,
			filemap: None,
		};

		this.init().map_err(CreationError::Init)?;

		Ok(this)
	}

	pub fn open(name: &'a str, mpath: PathBuf) -> Result<Self, PackageOpenError> {
		let cache = mpath.join("cache").join(name);

		if !cache.exists() {
			return Err(PackageOpenError::DoesNotExist);
		}

		let info = cache.join("pkg.toml");
		if !info.exists() {
			return Err(PackageOpenError::Malformed);
		}

		let mut toml = std::fs::File::open(info)?;
		let mut buf = String::new();
		toml.read_to_string(&mut buf)?;

		let data: PackageInfo = toml::from_str(&buf)?;
		let repo_url = url::Url::parse(data.repo_url)?;

		let repo_dir = cache.join("repo");

		Ok(Self {
			name,
			repo_url,
			mpath,

			cache,
			repo: repo_dir,
			filemap: None,
		})
	}
}
