use super::prelude::*;

pub fn build(x: &ArgMatches) -> Result<(), CommandError> {
	use colored::*;

	let pkg_name = x.value_of("PKG_NAME").unwrap();

	let mpath = get_pkg_dir(x)?;

	match Package::open(pkg_name, mpath) {
		Err(why) => println!(
			"Errored when opening package {}, [{}]",
			pkg_name.yellow(),
			why.to_string().red()
		),
		Ok(mut pkg) => match pkg.build() {
			Err(why) => println!(
				"Errored when building package {}, [{}]",
				pkg_name.yellow(),
				why.to_string().red()
			),
			Ok(_) => println!(
				"Built package {} {}!",
				pkg_name.yellow(),
				"successfully".green()
			),
		},
	};

	Ok(())
}
