use clap::Clap;

// Clone a module to a location.
#[derive(Clap)]
pub(crate) struct Clone {
	#[clap(short)]
	debug: bool
}

// Clone a module to a location.
#[derive(Clap)]
pub(crate) struct Update {
	#[clap(short)]
	debug: bool
}

#[derive(Clap)]
pub(crate) enum SubCommands {
	Clone(pkg::commands::Clone),
	Update(pkg::commands::Update)
}