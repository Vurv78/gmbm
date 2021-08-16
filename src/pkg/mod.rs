pub(crate) mod build;
pub(crate) mod process;
pub(crate) mod clone;

pub(crate) mod cli;

use std::path::PathBuf;

#[derive(Debug)]
pub enum PackageOpen {
	NotFound
}

use std::fmt;
impl fmt::Display for PackageOpen {
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
		write!(f, "{}", match self {
			PackageOpen::NotFound => "Package not found"
		})
	}
}

pub struct Package<'a> {
	pub name: &'a str, // Todo: Try and make this &'a str again
	pub repo: &'a str,
	pub mpath: PathBuf, // std::env::current_dir by default

	pub(crate) cache: PathBuf,
	pub(crate) filemap: Option<pelite::FileMap>
}

impl<'a> Package<'a> {
	pub fn new(name: &'a str, repo_url: &'a str, mpath: PathBuf) -> Self {
		let cache = mpath
			.join("cache")
			.join(name);

		Self {
			name: name,
			repo: repo_url,
			mpath: mpath,

			cache: cache,
			filemap: None
		}
	}

	pub fn open(name: &'a str, mpath: PathBuf) -> Result<Self, PackageOpen> {
		let cache = mpath
			.join("cache")
			.join(name);

		if !cache.exists() {
			return Err( PackageOpen::NotFound )
		}

		Ok(Self {
			name: name,
			repo: "", // Temporary
			mpath: mpath,

			cache: cache,
			filemap: None
		})
	}
}