use clap::ArgMatches;
use super::Package;

pub fn clone(x: &ArgMatches) -> Result<(), Box<dyn std::error::Error>>{
	use colored::*;

	let repo_url = x.value_of("REPO_URL").unwrap();
	let pkg_name = x.value_of("PKG_NAME").unwrap_or_else(|| {
		repo_url.split("/").last().unwrap()
	});

	let mut p = Package::new( pkg_name, repo_url, std::env::current_dir()? );
	if let Err(why) = p.clone() {
		println!( "Errored on clone: {}", why.to_string().red() )
	} else {
		println!("Cloned");
	}

	Ok(())
}