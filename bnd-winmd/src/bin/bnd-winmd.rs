//! CLI entry point for bnd-winmd.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// bnd-winmd — generate WinMD metadata from C headers.
#[derive(Parser, Debug)]
#[command(name = "bnd-winmd", version, about)]
struct Cli {
    /// Path to the bnd-winmd.toml configuration file.
    #[arg(default_value = "bnd-winmd.toml")]
    config: PathBuf,

    /// Output file path (overrides config).
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("bnd_winmd=info")),
        )
        .init();

    let cli = Cli::parse();
    bnd_winmd::run(&cli.config, cli.output.as_deref())?;
    Ok(())
}
