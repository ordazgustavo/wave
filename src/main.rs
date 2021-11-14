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

use crate::init::{init, InitFlags};
use crate::install::{install, InstallFlags};

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

#[paw::main]
fn main(args: Wave) -> anyhow::Result<()> {
    let mut term = Term::stdout();
    let now = Instant::now();

    let result = match args {
        Wave::Init { yes, name } => init(&mut term, name, InitFlags { yes })?,
        Wave::Install {
            development,
            exact,
            packages,
        } => install(packages, InstallFlags { development, exact })?,
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
