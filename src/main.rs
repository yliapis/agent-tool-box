use std::path::PathBuf;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};

use agent_tool_box::{Kind as SyncKind, apply, config, discover, plan};

#[derive(Parser)]
#[command(
    name = "atb",
    version,
    about = "Sync AI agent capabilities across agent harnesses"
)]
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
        Command::Sync(args) => sync(args),
    }
}

fn sync(args: SyncArgs) -> anyhow::Result<()> {
    if args.config.is_some() {
        anyhow::bail!("`--config` is not implemented yet — see docs/specs/config-spec.md");
    }
    let src = args.src.context("missing --src")?;
    let dst = args.dst.context("missing --dst")?;
    let dist = config::from_flags(src, dst, to_kind(args.kind));
    let catalog = discover(&dist.root, dist.kind)?;
    let plans = plan(&catalog, &dist.targets)?;
    apply(&plans)
}

fn to_kind(kind: Kind) -> SyncKind {
    match kind {
        Kind::Skill => SyncKind::Skill,
        Kind::Command => SyncKind::Command,
        Kind::Agent => SyncKind::Agent,
    }
}
