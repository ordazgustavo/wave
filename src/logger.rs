use std::io::Write;

use anyhow::{Context, Result};
use console::{style, Emoji};

use crate::WaveContext;

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub fn done(ctx: &WaveContext, elapsed: f32) -> Result<()> {
    writeln!(&ctx.term, "{} Done in {}s.", SPARKLE, elapsed).context("Failed to write to std")?;
    Ok(())
}

pub fn success(ctx: &WaveContext, message: &str) -> Result<()> {
    writeln!(
        &ctx.term,
        "{}",
        format!("{} {}", style("success").green(), message)
    )
    .context("Failed to write to std")?;
    Ok(())
}

pub fn warning(ctx: &WaveContext, message: &str) -> Result<()> {
    writeln!(
        &ctx.term,
        "{}",
        format!("{} {}", style("warning").yellow(), message)
    )
    .context("Failed to write to std")?;
    Ok(())
}
