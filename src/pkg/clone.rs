use super::Package;
use crate::util;
use std::{path::Path, process::Command};

#[derive(Debug)]
pub enum CloneError {
	Exists,
	Io(std::io::Error),
	Git(git2::Error)
}

impl std::fmt::Display for CloneError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			CloneError::Exists => "Already exists".to_owned(),
			CloneError::Io(e) => format!("IO Error: {}", e),
			CloneError::Git(e) => format!("Git clone failed with code: {}", e)
		})
	}
}

impl<'a> Package<'a> {
	pub fn clone(&mut self) -> Result<(), CloneError> {
		let cache_dir = &self.cache;

		let git_dir = cache_dir.join("repo");

		if git_dir.exists() {
			return Err( CloneError::Exists );
		}

		std::fs::create_dir_all(&git_dir).map_err(|x| CloneError::Io(x))?;

		// Clone repo of the package
		git2::Repository::clone_recurse(self.repo_url.as_str(), git_dir)
			.map_err(CloneError::Git)?;

		Ok(())
	}
}