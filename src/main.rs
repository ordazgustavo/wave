use std::time::Instant;

use console::Term;

mod add;
mod cli;
mod fs;
mod init;
mod logger;
mod package;
mod packument;
mod registry;

use crate::add::{add, AddFlags};
use crate::cli::{Command, Wave};
use crate::init::{init, InitFlags};

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
            Command::Add {
                development,
                exact,
                packages,
            } => add(&ctx, packages, AddFlags { development, exact }).await?,
            _ => todo!(),
        },
        None => {
            add(
                &ctx,
                Vec::new(),
                AddFlags {
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
