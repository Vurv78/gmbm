use std::path::PathBuf;

pub fn current_exe_dir() -> std::io::Result<PathBuf> {
	let mut path = std::env::current_exe()?;
	path.pop();

	Ok(path)
}
