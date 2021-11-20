use std::time::Instant;

use console::Term;

mod cli;
mod fs;
mod init;
mod install;
mod logger;
mod package;
mod packument;
mod registry;

use crate::cli::{Command, Wave};
use crate::init::{init, InitFlags};
use crate::install::{install, InstallFlags};

pub struct WaveContext {
    pub term: Term,
    pub client: reqwest::Client,
}

#[paw::main]
#[tokio::main]
async fn main(args: Wave) -> anyhow::Result<()> {
    let term = Term::stdout();
    let client = reqwest::Client::new();
    let ctx = WaveContext { term, client };
    let now = Instant::now();

    let cmd = args.cmd;

    match cmd {
        Some(cmd) => match cmd {
            Command::Init { yes, name } => init(&ctx, name, InitFlags { yes })?,
            Command::Install {
                development,
                exact,
                packages,
            } => install(&ctx, packages, InstallFlags { development, exact }).await?,
            _ => todo!(),
        },
        None => {
            install(
                &ctx,
                Vec::new(),
                InstallFlags {
                    development: false,
                    exact: false,
                },
            )
            .await?
        }
    };

    logger::done(&ctx, now.elapsed().as_secs_f32())?;

    Ok(())
}
