use super::Package;

use std::process::Command;

#[derive(Debug)]
pub enum BuildError {
	AlreadyBuilt,
	CommandFailed(std::io::Error),
	Io(std::io::Error)
}

impl std::fmt::Display for BuildError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			BuildError::AlreadyBuilt => "already built".to_owned(),
			BuildError::CommandFailed(err) => format!("command failed: {}", err),
			BuildError::Io(err) => format!("io error: {}", err),
		})
	}
}

impl<'a> Package<'a> {
	pub fn build(&mut self) -> Result<(), BuildError> {
		let cache_dir = &self.cache;

		let main_path = cache_dir
			.join(self.name)
			.join("main.cpp");

		let out_path = cache_dir
			.join("main.dll");

		if out_path.exists() {
			return Err( BuildError::AlreadyBuilt );
		}

		std::fs::create_dir_all(&cache_dir).map_err(|x| BuildError::Io(x))?;

		// Clone repo of the package
		let status = Command::new("git")
			.current_dir( &cache_dir )
			.args( ["clone", self.repo, self.name, "--recurse-submodules"] )
			.status()
			.map_err(|x| BuildError::CommandFailed(x))?;

		if !status.success() {
			anyhow::anyhow!( "Exit with code {}", status.code().unwrap_or(-1) );
		}

		// TODO: Use .exit_ok()? when it's stabilized.

		// Compile main.cpp to main.dll in the package.
		let status = Command::new("gcc")
			.args( [&main_path.display().to_string(), "-o", &out_path.display().to_string(), "-shared"] )
			.status()
			.map_err(|x| BuildError::CommandFailed(x))?;

		if !status.success() {
			anyhow::anyhow!( "Exit with code {}", status.code().unwrap_or(-1) );
		}

		self.filemap = Some( pelite::FileMap::open(&out_path).map_err(|x| BuildError::Io(x))? );
		Ok(())
	}
}