use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};

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

/// Exactly one of `--config` or the `--tool --src --dst` trio; see docs/specs/atb-sync.md.
#[derive(Args)]
#[command(group(ArgGroup::new("mode").required(true).args(["config", "tool"])))]
struct SyncArgs {
    /// Path to a sync.yaml config
    #[arg(long, conflicts_with_all = ["tool", "src", "dst"])]
    config: Option<PathBuf>,

    /// Target tool
    #[arg(long, requires_all = ["src", "dst"])]
    tool: Option<Tool>,

    /// Source tree searched for SKILL.md
    #[arg(long, requires = "tool")]
    src: Option<PathBuf>,

    /// Output directory for skills
    #[arg(long, requires = "tool")]
    dst: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Tool {
    Claude,
    Cursor,
    Codex,
    Opencode,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sync(_) => {
            anyhow::bail!("`atb sync` is not implemented yet — spec in flight (docs/specs/atb-sync.md)")
        }
    }
}
