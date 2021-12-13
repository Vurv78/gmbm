mod commands;
pub(crate) use crate::pkg::Package;
use clap::{App, AppSettings, Arg};

#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
	#[error("{0}")]
	Clap(#[from] clap::Error),

	#[error("{0}")]
	Command(#[from] commands::CommandError),
}

pub fn process() -> Result<(), ProcessError> {
	let opts = App::new( clap::crate_name!() )
		.about( clap::crate_description!() )
		.version( clap::crate_version!() )
		.author( clap::crate_authors!() )
		.alias("gm" )
		.subcommands(vec![
			App::new("init")
				.about("Initializes the package manager at the exe's location"),

			App::new("clone")
				.about("Clones a binary module repo without building it.")
				.arg(
					Arg::new("REPO_URL")
						.help("Sets the input repo to use. Ex: https://github.com/Vurv78/Autorun-rs")
						.required(true)
				)
				.arg(
					Arg::new("PKG_NAME")
						.short('n')
						.help("Sets the output name and folder to use for the package. By default it is the name of the github repo")
						.required(false)
				)
				.arg(
					Arg::new("PKG_DIR")
						.short('p')
						.help("Sets the directory where the packages will be found. Defaults to exe dir")
						.required(false)
				),

			App::new("build")
				.about("Builds a binary module from a repo")
				.arg(
					Arg::new("PKG_NAME")
						.help("Sets the input repo to use. Ex: gm_bromsock")
						.required(true)
				)
				.arg(
					Arg::new("PKG_DIR")
						.short('p')
						.help("Sets the directory where the packages will be found. Defaults to exe dir")
						.required(false)
				),

			App::new("verify")
				.about("Verifies that a binary module built correctly")
				.arg(
					Arg::new("PKG_NAME")
						.help("Sets the package to be verified. Must first build it.")
						.required(true)
				)
				.arg(
					Arg::new("PKG_DIR")
						.short('p')
						.help("Sets the directory where the packages will be found. Defaults to exe dir")
						.required(false)
				),

			App::new("install")
				.about("Installs a built binary module to your garrysmod/lua/bin folder")
				.arg(
					Arg::new("PKG_NAME")
						.help("The package's build to install into garrysmod")
						.required(true)
				)
				.arg(
					Arg::new("PKG_DIR")
						.short('p')
						.help("Sets the directory where the packages will be found. Defaults to exe dir")
						.required(false)
				)
				.arg(
					Arg::new("GMOD_DIR")
						.short('d')
						.long("gmod_dir")
						.help("Sets the directory where gmod will be found. By default it searches for garrysmod")
						.required(false)
				)
				.arg(
					Arg::new("REALM")
						.short('r')
						.long("realm")
						.help("Sets the realm of the package. Defaults to 'cl' for client")
						.required(false)
						.possible_values(&["cl", "sv"])
				)
		])
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.get_matches();

	match opts.subcommand() {
		Some(("init", x)) => commands::init(x)?,
		Some(("clone", x)) => commands::clone(x)?,
		Some(("build", x)) => commands::build(x)?,
		Some(("verify", x)) => commands::verify(x)?,
		Some(("install", x)) => commands::install(x)?,
		_ => (),
	}

	Ok(())
}
