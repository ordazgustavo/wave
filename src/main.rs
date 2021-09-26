use std::io::Write;
use std::time::Instant;

use console::{Emoji, Term};

mod cli;
mod fs;
mod init;
mod install;
mod logger;
mod package;

use cli::Wave;

use crate::init::init;
use crate::install::install;

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

#[paw::main]
fn main(args: Wave) -> anyhow::Result<()> {
    let term = Term::stdout();
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

    writeln!(&term, "{}", result)?;
    writeln!(
        &term,
        "{} Done in {}s.",
        SPARKLE,
        now.elapsed().as_secs_f32()
    )?;

    Ok(())
}
