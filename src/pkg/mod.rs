pub(crate) mod build;
pub(crate) mod process;
pub(crate) mod clone;
pub(crate) mod init;

use anyhow::bail;

use std::{io::Read, path::PathBuf};
use url::Url;

pub struct Package<'a> {
	pub name: &'a str,
	pub repo_url: Url,
	pub mpath: PathBuf, // Current exe dir by default

	pub cache: PathBuf,
	pub repo: PathBuf,
	pub(crate) filemap: Option<pelite::FileMap>
}

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct PackageInfo<'a> {
	name: &'a str,
	repo_url: &'a str
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

impl<'a> Package<'a> {
	pub fn create(name: &'a str, repo_url: Url, mpath: PathBuf) -> Result<Self, CreationError> {
		let cache = mpath
			.join("cache")
			.join(name);

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
			filemap: None
		};

		this.init().map_err(CreationError::Init)?;

		Ok(this)
	}

	pub fn open(name: &'a str, mpath: PathBuf) -> anyhow::Result<Self> {
		let cache = mpath
			.join("cache")
			.join(name);

		if !cache.exists() {
			bail!("Package {} doesn't exist", name)
		}

		let info = cache.join("pkg.toml");
		if !info.exists() {
			bail!("Malformed package {}", name);
		}

		let mut toml = std::fs::File::open(info)?;
		let mut buf = String::new();
		toml.read_to_string(&mut buf)?;

		let data: PackageInfo = toml::from_str(&buf)?;
		let repo_url = url::Url::parse(  data.repo_url )?;

		let repo_dir = cache.join("repo");

		Ok(Self {
			name,
			repo_url,
			mpath,

			cache,
			repo: repo_dir,
			filemap: None
		})
	}
}