use clap::ArgMatches;
use super::Package;

pub fn clone(x: &ArgMatches) -> Result<(), Box<dyn std::error::Error>>{
	use colored::*;

	let repo_url = x.value_of("REPO_URL").unwrap();
	let pkg_name = x.value_of("PKG_NAME").unwrap_or_else(|| {
		repo_url.split("/").last().unwrap()
	});

	match url::Url::parse(repo_url) {
		Ok(url) => {
			match Package::create( pkg_name, url, crate::util::current_exe_dir()? ) {
				Ok(mut pkg) => {
					if let Err(why) = pkg.clone() {
						println!( "Errored on clone: {}", why.to_string().red().bold() )
					} else {
						println!("Cloned {} successfully", repo_url);
					}
				},

				Err(e) => println!("{}", e.to_string().red().bold())
			}
		},
		Err(e) => {
			println!("{}: {}", "Invalid URL".red(), e);
		}
	}

	Ok(())
}