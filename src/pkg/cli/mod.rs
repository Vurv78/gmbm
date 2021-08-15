pub fn process() -> Result<(), Box<dyn std::error::Error>>{
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
			(@arg dir: -d ... "Sets the directory where the packages will be found. Defaults to std::env::current_dir")
		)
	).get_matches();

	match opts.subcommand() {
		Some( ("clone", x) ) => {
			let repo_url = x.value_of("REPO_URL").unwrap();
			let pkg = x.value_of("PKG_NAME").unwrap();

			let mut pkg = Package::new( pkg, repo_url, std::env::current_dir()? );
			if let Err(why) = pkg.build() {
				error!("{}", why);
			}
		},
		Some( ("verify", x) ) => {
			let pkg_name = x.value_of("PKG_NAME").unwrap();
			let dir = match x.value_of("dir") {
				Some(path) => path.into(),
				None => std::env::current_dir()?
			};

			println!( "Verifying {}", pkg_name );

			let pkg = Package::open(pkg_name, dir)?;
			if let Err(why) = pkg.verify() {
				error!("{}", why);
			}
		},
		_ => ()
	}

	Ok(())
}