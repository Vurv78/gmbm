use super::Package;
mod gcc;
mod msbuild;

#[derive(Debug)]
pub enum BuildError {
	AlreadyBuilt,
	Io(std::io::Error),
	NoMSBuild,
	Missing,
	CommandFailure(i32)
}

impl std::fmt::Display for BuildError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			BuildError::AlreadyBuilt => "Already Built".to_owned(),
			BuildError::Io(err) => format!("IO Error: {}", err),
			BuildError::NoMSBuild => "Couldn't find msbuild, make sure you have visual studio/msbuild installed!".to_owned(),
			BuildError::Missing => "Package doesn't exist, clone it first.".to_owned(),
			BuildError::CommandFailure(err) => format!("Command failed with code: {}", err),
		})
	}
}

impl<'a> Package<'a> {
	pub fn build(&mut self) -> Result<(), BuildError> {
		let cache_dir = &self.cache;

		if !cache_dir.exists() {
			return Err( BuildError::Missing );
		}

		let out_path = cache_dir
			.join("main.dll");

		if out_path.exists() {
			return Err( BuildError::AlreadyBuilt );
		}

		let main_path = cache_dir
			.join(self.name)
			.join("main.cpp");

		// TODO: Use .exit_ok()? when it's stabilized.
		match msbuild::try_compile( cache_dir, &main_path, &out_path ) {
			Ok(_) => println!("Successfully compiled."),
			Err(why) => {
				return Err(why);
			},
		}

		self.filemap = Some( pelite::FileMap::open(&out_path).map_err(|x| BuildError::Io(x))? );
		Ok(())
	}
}