pub(crate) mod build;
pub(crate) mod process;
pub(crate) mod clone;
pub(crate) mod cli;

use anyhow::bail;

use std::{io::{Read, Write}, path::PathBuf};
use url::Url;

pub struct Package<'a> {
	pub name: &'a str,
	pub repo_url: Url,
	pub mpath: PathBuf, // std::env::current_dir by default

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

impl<'a> Package<'a> {
	pub fn create(name: &'a str, repo_url: Url, mpath: PathBuf) -> anyhow::Result<Self> {
		let cache = mpath
			.join("cache")
			.join(name);

		if cache.exists() {
			bail!("Package already exists");
		}

		let repo_dir = cache.join("repo");

		if let Err(why) = std::fs::create_dir_all(&cache) {
			bail!("IO Error: {}", why);
		}

		match std::fs::File::create(cache.join("pkg.toml")) {
			Ok(mut f) => {
				let a = PackageInfo {
					name: name,
					repo_url: repo_url.as_str()
				};
				let b = toml::to_string(&a).unwrap();
				f.write_all(b.as_bytes()).unwrap();
			}
			Err(why) => {
				bail!("Error when creating pkg.toml. {}", why);
			}
		}

		Ok(Self {
			name: name,
			repo_url: repo_url,
			mpath: mpath,

			cache: cache,
			repo: repo_dir,
			filemap: None
		})
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
			name: name,
			repo_url: repo_url,
			mpath: mpath,

			cache: cache,
			repo: repo_dir,
			filemap: None
		})
	}
}