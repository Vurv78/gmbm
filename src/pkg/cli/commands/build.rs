use clap::ArgMatches;
use super::Package;

pub fn build(x: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
	use colored::*;

	let pkg_name = x.value_of("PKG_NAME").unwrap();

	let mpath = match x.value_of("dir") {
		Some(path) => path.into(),
		None => crate::util::current_exe_dir()?
	};

	match Package::open(pkg_name, mpath) {
		Err(why) => println!( "Errored when opening package {}, [{}]", pkg_name.yellow(), why.to_string().red() ),
		Ok(mut pkg) => {
			match pkg.build() {
				Err(why) => println!( "Errored when building package {}, [{}]", pkg_name.yellow(), why.to_string().red() ),
				Ok(_) => println!( "Built package {} {}!", pkg_name.yellow(), "successfully".green() )
			}
		}
	};

	Ok(())
}