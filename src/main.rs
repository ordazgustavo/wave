use std::io::{self, Write};
use std::time::Instant;

use structopt::StructOpt;

mod cli;
mod fs;
mod init;
mod install;
mod logger;
mod package;

use cli::Wave;

use crate::init::init;
use crate::install::install;

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let args = Wave::from_args();

    let now = Instant::now();

    let result = match args {
        Wave::Init { yes: _, name } => init(name)?,
        Wave::Install {
            development: _,
            exact: _,
            packages,
        } => install(packages)?,
        // Wave::List { packages } => todo!(),
        // Wave::Uninstall { packages } => todo!(),
        _ => todo!(),
    };

    writeln!(handle, "{}", result)?;
    writeln!(handle, "✨ Done in {}s.", now.elapsed().as_secs_f32())?;

    Ok(())
}
