pub fn process() -> Result<(), Box<dyn std::error::Error>>{
	use colored::*;
	use super::Package;

	let opts = clap::clap_app!(myapp =>
		(version: "0.1.0")
		(author: "Vurv78 <vurvdevelops@gmail.com>")
		(about: "Allows you to download garrysmod binary modules in one neat hub.")
		(@subcommand clone =>
			(about: "Clones a binary module repo")
			(@arg REPO_URL: +required "Sets the input repo to use. Ex: https://github.com/Vurv78/Autorun-rs")
			(@arg PKG_NAME: +required "Sets the output name and folder to use for the binpkg")
		)
		(@subcommand verify =>
			(about: "Verifies that a binary module built correctly")
			(@arg PKG_NAME: +required "Sets the pkg to verify, same as in clone")
			(@arg pkg_dir: -pd ... "Sets the directory where the packages will be found. Defaults to std::env::current_dir")
		)
		(@subcommand install =>
			(about: "Builds and installs a binary module to your garrysmod/lua/bin folder.")
			(@arg PKG_NAME: +required "The pkg to send, same as in clone")
			(@arg pkg_dir: -pd ... "Sets the directory where the packages will be found. Defaults to std::env::current_dir")
			(@arg gmod_dir: -d ... "Sets the gmod directory to replace the module in. By default scans your filesystem for gmod.")
		)
	).get_matches();

	match opts.subcommand() {
		Some( ("clone", x) ) => {
			let repo_url = x.value_of("REPO_URL").unwrap();
			let pkg_name = x.value_of("PKG_NAME").unwrap();

			let mut pkg = Package::new( pkg_name, repo_url, std::env::current_dir()? );
			if let Err(why) = pkg.build() {
				error!("Errored when building package {}, '{}'", pkg_name, why);
			}
		},

		Some( ("verify", x) ) => {

			let pkg_name = x.value_of("PKG_NAME").unwrap();
			let dir = match x.value_of("dir") {
				Some(path) => path.into(),
				None => std::env::current_dir()?
			};

			match Package::open(pkg_name, dir) {
				Ok(pkg) => {
					if let Err(why) = pkg.verify() {
						println!( "Package {} {}. [{}]", pkg_name.yellow(), "failed".red(), why.to_string().yellow() );
					} else {
						println!( "Package {} {}!", pkg_name.yellow(), "verified".green() );
					}
				}
				Err(why) => {
					println!( "{} to open package {}. [{}]", "Failed".red(), pkg_name.yellow(), why.to_string().yellow() );
				}
			}
		},

		Some( ("install", x) ) => {
			use std::path::PathBuf;

			let pkg_name = x.value_of("PKG_NAME").unwrap();
			let dir = match x.value_of("dir") {
				Some(path) => path.into(),
				None => std::env::current_dir()?
			};

			fn find_gmod_dir() -> Option<PathBuf> {
				const STEAM_FOLDERS: &[&str] = &[
					r#"C:\Program Files (x86)\Steam"#,
					r#"C:\SteamLibrary"#,
					r#"D:\SteamLibrary"#,
					r#"E:\SteamLibrary"#,
					r#"F:\SteamLibrary"#,
					r#"G:\SteamLibrary"#,
				];
				for steam_dir in STEAM_FOLDERS {
					let gmod_dir = PathBuf::from(steam_dir)
						.join("steamapps")
						.join("common")
						.join("GarrysMod");
					if gmod_dir.exists() {
						return Some(gmod_dir);
					}
				}
				None
			}

			// Handle gmod path
			// Creates garrysmod/lua/bin if missing, then copies the pkg to gmsv
			fn handle(pkg_name: &str, mpath: PathBuf, p: PathBuf) {
				let bin_dir = p
					.join("garrysmod")
					.join("lua")
					.join("bin");

				if !bin_dir.is_dir() {
					println!("Bin directory @{} didn't exist, creating!", bin_dir.display().to_string().yellow());
					if let Err(why) = std::fs::create_dir_all(&bin_dir) {
						error!("Failed to create dir {} [{}]. Aborting install!", bin_dir.display(), why);
						return;
					}
				}

				match Package::open(pkg_name, mpath) {
					Ok(mut p) => {
						// Replace with p.is_built() or something in the future.
						let dll_path = p.cache.join("main.dll");
						if !dll_path.exists() {
							println!("{} doesn't exist, building...", dll_path.display().to_string().yellow());
							if let Err(why) = p.build() {
								println!("Error when building. {}", why.to_string().red());
							}
						}

						// Assume it exists now.
						if let Err(why) = std::fs::copy(dll_path, bin_dir) {
							error!("{}", why);
						}
					},
					Err(why) => error!("Failed to open package {}. [{}]", pkg_name, why.to_string()),
				};
			}

			match x.value_of("gmod_dir") {
				Some(gmod_dir) => handle(pkg_name, dir, gmod_dir.into()),
				None => {
					match find_gmod_dir() {
						Some(gmod_dir) => handle(pkg_name, dir, gmod_dir),
						None => error!("Couldn't find your garrysmod dir. Set the -d or --gmod_dir flags.")
					}
				}
			};
		}
		_ => ()
	}

	Ok(())
}