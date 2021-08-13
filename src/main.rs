use std::process::Command;

const GMOD_DLLOPEN: &str = "gmod13_open";
const GMOD_DLLCLOSE: &str = "gmod13_close";

#[macro_use] extern crate log;

struct Package {
	name: String, // Todo: Try and make this &'a str again
	filemap: pelite::FileMap
}

fn build_pkg<'a, S: 'a + AsRef<str>>(name: S, repo: S) -> Result<Package, std::io::Error> {
	let repo = repo.as_ref();

	let cache_dir = std::env::current_dir()?
		.join("cache")
		.join(name.as_ref());

	let main_path = cache_dir
		.join(name.as_ref())
		.join("main.cpp");

	let out_path = cache_dir
		.join("main.dll");

	if out_path.exists() {
		return Ok( Package {
			name: name.as_ref().to_owned(),
			filemap: pelite::FileMap::open(&out_path)?
		} );
	}

	std::fs::create_dir_all(&cache_dir)?;

	// Clone repo of the package
	Command::new("git")
		.current_dir( &cache_dir )
		.args( ["clone", repo, name.as_ref(), "--recurse-submodules"] )
		.status()?;

	// Compile main.cpp to main.dll in the package.
	Command::new("gcc")
		.args( [&main_path.display().to_string(), "-o", &out_path.display().to_string(), "-shared"] )
		.status()?;

	Ok( Package {
		name: name.as_ref().to_owned(),
		filemap: pelite::FileMap::open(&out_path)?
	} )
}

fn handle_pkg(p: &Package) -> anyhow::Result<()> {
	let pe = pelite::PeFile::from_bytes( p.filemap.as_ref() )?;

	if let Err(_) = pe.get_export_by_name(GMOD_DLLOPEN) {
		error!("Package {} missing {} export.", p.name, GMOD_DLLOPEN);
	}

	if let Err(_) = pe.get_export_by_name(GMOD_DLLCLOSE) {
		error!("Package {} missing {} export.", p.name, GMOD_DLLCLOSE);
	}

	Ok(())
}
// TODO: CLI
fn main() -> anyhow::Result<()> {
	let path = build_pkg("example", "https://github.com/Vurv78/ExampleModule")?;
	handle_pkg(&path)?;

	println!("Hello, world!");

	Ok(())
}
