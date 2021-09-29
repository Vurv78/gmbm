use clap::ArgMatches;
use super::Package;

pub fn verify(x: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
	use colored::*;

	let pkg_name = x.value_of("PKG_NAME").unwrap();
	let dir = match x.value_of("dir") {
		Some(path) => path.into(),
		None => crate::util::current_exe_dir()?
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

	Ok(())
}