use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "atb", version, about = "Sync AI agent capabilities across agent harnesses")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Discover skills under a source tree and copy them into tool output dirs
    Sync(SyncArgs),
}

/// `--src` and `--dst` are required. `--config` is optional and fails when present.
#[derive(Args)]
struct SyncArgs {
    /// Path to a sync.yaml config (not implemented; exits non-zero)
    #[arg(long)]
    config: Option<PathBuf>,

    /// Source tree searched for artifacts
    #[arg(long, required_unless_present = "config", requires = "dst")]
    src: Option<PathBuf>,

    /// Output directory
    #[arg(long, required_unless_present = "config", requires = "src")]
    dst: Option<PathBuf>,

    /// Artifact kind
    #[arg(long, default_value = "skill")]
    kind: Kind,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Kind {
    Skill,
    Command,
    Agent,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sync(args) => {
            if args.config.is_some() {
                anyhow::bail!(
                    "`--config` is not implemented yet — see docs/specs/config-spec.md"
                );
            }
            anyhow::bail!(
                "`atb sync` is not implemented yet — spec in flight (docs/specs/atb-sync.md)"
            )
        }
    }
}
