
use std::{ffi::OsStr, path::{Path, PathBuf}};

pub(crate) fn current_exe_dir() -> std::io::Result<PathBuf> {
	let mut path = std::env::current_exe()?;
	path.pop();

	Ok(path)
}

// Gets the first file in a directory of a certain extension.
pub(crate) fn get_first_of_ext<P: AsRef<Path>, Ext: AsRef<OsStr>>(path: P, ext: Ext) -> Option<PathBuf> {
	let path = path.as_ref();
	let ext = ext.as_ref();

	match std::fs::read_dir(path) {
		Ok(dir) => {
			for entry in dir.flatten() {
				let entry_path = entry.path();
				if let Some(ext2) = entry_path.extension() {
					if ext2 == ext {
						return Some(entry_path);
					}
				}
			}
			None
		}
		Err(_) => None,
	}
}