// `8lackout` — turns any device into a mesh node. PLACEHOLDER build.
use clap::{Parser, Subcommand};

mod config;
mod identity;
mod commands;

#[derive(Parser)]
#[command(name = "8lackout", version, about = "Coordinate when the lights go out")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Join the mesh
    Up { #[arg(long, default_value = "node")] name: String },
    /// Post a need
    Need { text: String },
    /// List live needs nearby
    Board,
    /// Claim a need
    Claim { id: String },
    /// Co-sign proof-of-aid on handoff
    Deliver { id: String, #[arg(long)] with: Option<String> },
    /// Run a beacon node
    Beacon { #[arg(long)] solar: bool },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Up { name } => commands::up::run(&name).await,
        Cmd::Need { text } => commands::need::run(&text).await,
        Cmd::Board => commands::board::run().await,
        Cmd::Claim { id } => commands::claim::run(&id).await,
        Cmd::Deliver { id, with } => commands::deliver::run(&id, with.as_deref()).await,
        Cmd::Beacon { solar } => commands::beacon::run(solar).await,
    }
}
