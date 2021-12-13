use super::prelude::*;

pub fn verify(x: &ArgMatches) -> Result<(), CommandError> {
	use colored::*;

	let pkg_name = x.value_of("PKG_NAME").unwrap();
	let dir = get_pkg_dir(x)?;

	match Package::open(pkg_name, dir) {
		Ok(pkg) => {
			if let Err(why) = pkg.verify() {
				println!(
					"Package {} {}. [{}]",
					pkg_name.yellow(),
					"failed".red(),
					why.to_string().yellow()
				);
			} else {
				println!("Package {} {}!", pkg_name.yellow(), "verified".green());
			}
		}
		Err(why) => {
			println!(
				"{} to open package {}. [{}]",
				"Failed".red(),
				pkg_name.yellow(),
				why.to_string().yellow()
			);
		}
	}

	Ok(())
}
