
use clap::ArgMatches;
use super::Package;

pub fn install(x: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
	use std::path::PathBuf;
	use colored::*;

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

	Ok(())
}