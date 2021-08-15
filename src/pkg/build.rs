use super::Package;

use std::{
	process::Command
};

impl<'a> Package<'a> {
	pub fn build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let cache_dir = &self.cache;

		let main_path = cache_dir
			.join(self.name)
			.join("main.cpp");

		let out_path = cache_dir
			.join("main.dll");

		if out_path.exists() {
			return Ok(()); // Already built
		}

		std::fs::create_dir_all(&cache_dir)?;

		// Clone repo of the package
		Command::new("git")
			.current_dir( &cache_dir )
			.args( ["clone", self.repo, self.name, "--recurse-submodules"] )
			.status()?;

		// Compile main.cpp to main.dll in the package.
		Command::new("gcc")
			.args( [&main_path.display().to_string(), "-o", &out_path.display().to_string(), "-shared"] )
			.status()?;

		self.filemap = Some( pelite::FileMap::open(&out_path)? );
		Ok(())
	}
}