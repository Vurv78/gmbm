mod commands;
pub(crate) use super::Package;

pub fn process() -> Result<(), Box<dyn std::error::Error>>{
	use colored::*;

	let opts = clap::clap_app!(myapp =>
		(version: "0.1.0")
		(author: "Vurv78 <vurvdevelops@gmail.com>")
		(about: "Allows you to download garrysmod binary modules in one neat hub.")
		(@subcommand init =>
			(about: "Initializes the package manager at the cwd.")
		)
		(@subcommand clone =>
			(about: "Clones a binary module repo, without building it.")
			(@arg REPO_URL: +required "Sets the input repo to use. Ex: https://github.com/Vurv78/Autorun-rs")
			(@arg PKG_NAME: "Sets the output name and folder to use for the binpkg. By default it is the name of the github repo")
		)
		(@subcommand build =>
			(about: "Builds a binary module from a repo.")
			(@arg PKG_NAME: +required "Sets the input repo to use. Ex: gm_bromsocket")
			(@arg pkg_dir: -p ... "Sets the directory where the packages will be found. Defaults to std::env::current_dir")
		)
		(@subcommand verify =>
			(about: "Verifies that a binary module built correctly")
			(@arg PKG_NAME: +required "Sets the pkg to verify, same as in clone")
			(@arg pkg_dir: -p ... "Sets the directory where the packages will be found. Defaults to std::env::current_dir")
		)
		(@subcommand install =>
			(about: "Builds and installs a binary module to your garrysmod/lua/bin folder.")
			(@arg PKG_NAME: +required "The pkg to send, same as in clone")
			(@arg pkg_dir: -p ... "Sets the directory where the packages will be found. Defaults to std::env::current_dir")
			(@arg gmod_dir: -d ... "Sets the gmod directory to replace the module in. By default scans your filesystem for gmod.")
			(@arg realm: -r ... "Sets the realm to install. Either 'sv' or 'cl' for Serverside and Clientside respectively.")
		)
	).get_matches();

	match opts.subcommand() {
		Some( ("init", x) ) => commands::init(x)?,
		Some( ("clone", x) ) => commands::clone(x)?,
		Some( ("build", x) ) => commands::build(x)?,
		Some( ("verify", x) ) => commands::verify(x)?,
		Some( ("install", x) ) => commands::install(x)?,
		_ => (),
	}

	Ok(())
}