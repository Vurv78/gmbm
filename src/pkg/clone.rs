use super::Package;

#[derive(Debug, thiserror::Error)]
pub enum CloneError {
	#[error("Already exists")]
	Exists,
	#[error("IO Error `{0}`")]
	Io(std::io::Error),
	#[error("Git clone failed with error: `{0}`")]
	Git(git2::Error),
}

impl<'a> Package<'a> {
	pub fn clone(&mut self) -> Result<(), CloneError> {
		let cache_dir = &self.cache;

		let git_dir = cache_dir.join("repo");

		if git_dir.exists() {
			return Err(CloneError::Exists);
		}

		std::fs::create_dir_all(&git_dir).map_err(CloneError::Io)?;

		// Clone repo of the package
		git2::Repository::clone_recurse(self.repo_url.as_str(), git_dir)
			.map_err(CloneError::Git)?;

		Ok(())
	}
}
