pub fn process() -> Result<(), Box<dyn std::error::Error>>{
	use colored::*;
	use super::Package;

	let opts = clap::clap_app!(myapp =>
		(version: "0.1.0")
		(author: "Vurv78 <vurvdevelops@gmail.com>")
		(about: "Allows you to download garrysmod binary modules in one neat hub.")
		(@subcommand clone =>
			(about: "Clones a binary module repo, without building it.")
			(@arg REPO_URL: +required "Sets the input repo to use. Ex: https://github.com/Vurv78/Autorun-rs")
			(@arg PKG_NAME: +required "Sets the output name and folder to use for the binpkg")
		)
		(@subcommand build =>
			(about: "Builds a binary module from a repo.")
			(@arg PKG_NAME: +required "Sets the input repo to use. Ex: vistrace")
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
		Some( ("clone", x) ) => {
			let repo_url = x.value_of("REPO_URL").unwrap();
			let pkg_name = x.value_of("PKG_NAME").unwrap();

			let mut p = Package::new( pkg_name, repo_url, std::env::current_dir()? );
			if let Err(why) = p.clone() {
				println!( "Errored on clone: {}", why.to_string().red() )
			} else {
				println!("Cloned");
			}
		},

		Some( ("build", x) ) => {
			let pkg_name = x.value_of("PKG_NAME").unwrap();

			let mpath = match x.value_of("dir") {
				Some(path) => path.into(),
				None => std::env::current_dir()?
			};

			match Package::open(pkg_name, mpath) {
				Err(why) => println!( "Errored when opening package {}, [{}]", pkg_name.yellow(), why.to_string().red() ),
				Ok(mut pkg) => {
					match pkg.build() {
						Err(why) => println!( "Errored when building package {}, [{}]", pkg_name.yellow(), why.to_string().red() ),
						Ok(_) => println!( "Built package {} {}!", pkg_name.yellow(), "successfully".green() )
					}
				}
			}
		}

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

			// Package main dir
			let dir = match x.value_of("dir") {
				Some(path) => path.into(),
				None => std::env::current_dir()?
			};

			// Gmod realm. Defaults to client
			let realm = match x.value_of("realm") {
				Some(realm) => realm,
				None => "cl"
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
			fn install(pkg_name: &str, mpath: PathBuf, gmod_dir: PathBuf, realm: &str) {
				let bin_dir = gmod_dir
					.join("garrysmod")
					.join("lua")
					.join("bin");

				if !bin_dir.is_dir() {
					println!("Bin directory @{} didn't exist, creating!", bin_dir.display().to_string().yellow());
					if let Err(why) = std::fs::create_dir_all(&bin_dir) {
						println!( "Failed to create dir {} [{}]. Aborting install!", bin_dir.display().to_string().yellow(), why.to_string().red() );
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

						let final_dir = bin_dir
							.join( format!("gm{}_{}_{}{}.dll", realm, p.name, "win", std::mem::size_of::<usize>() * 8 ) );

						// Assume it exists now.
						if let Err(why) = std::fs::copy(&dll_path, &final_dir) {
							println!("Failed to copy dll {} to {}, [{}]", dll_path.display().to_string().yellow(), final_dir.display().to_string().yellow(), why.to_string().red() );
						} else {
							println!( "Installed to {}", final_dir.display().to_string().yellow() );
						}
					},
					Err(why) => println!( "Failed to open package {}. [{}]", pkg_name.yellow(), why.to_string().red() ),
				};
			}

			match x.value_of("gmod_dir") {
				Some(gmod_dir) => install(pkg_name, dir, gmod_dir.into(), realm),
				None => {
					match find_gmod_dir() {
						Some(gmod_dir) => install(pkg_name, dir, gmod_dir, realm),
						None => println!("Couldn't find your garrysmod dir. Set the {} or {} flags.", "-d".yellow(), "--gmod_dir".yellow())
					}
				}
			};
		}
		_ => ()
	}

	Ok(())
}