#[derive(Debug, thiserror::Error)]
pub enum CommandError {
	#[error("IO Error: {0}")]
	IOError(#[from] std::io::Error),
}

pub(crate) mod prelude {
	pub(crate) use super::{CommandError, Package};
	pub(crate) use clap::ArgMatches;

	pub(crate) fn get_pkg_dir(args: &ArgMatches) -> Result<std::path::PathBuf, std::io::Error> {
		match args.value_of("PKG_DIR") {
			Some(path) => Ok(path.into()),
			None => crate::util::current_exe_dir(),
		}
	}
}

mod build;
mod clone;
mod init;
mod install;
mod verify;

pub(crate) use super::Package;

pub use build::build;
pub use clone::clone;
pub use init::init;
pub use install::install;
pub use verify::verify;
