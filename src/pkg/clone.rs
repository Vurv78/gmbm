use super::Package;
use std::process::Command;

#[derive(Debug)]
pub enum CloneError {
	Exists,
	Io(std::io::Error),
	Git(i32)
}

impl std::fmt::Display for CloneError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			CloneError::Exists => "Already exists".to_owned(),
			CloneError::Io(e) => format!("IO Error: {}", e),
			CloneError::Git(code) => format!("Git clone failed with code: {}", code)
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
		let status = Command::new("git")
			.current_dir( &cache_dir )
			.args( ["clone", self.repo_url.as_str(), "repo", "--recurse-submodules"] )
			.status()
			.map_err(|x| CloneError::Io(x))?;

		if !status.success() {
			return Err( CloneError::Git( status.code().unwrap_or(-1) ) );
		}

		Ok(())
	}
}