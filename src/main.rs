mod apps;
mod config;
mod daemon;
mod diagnostics;
mod logs;
mod packages;
mod services;
mod system;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "triad", version, about = "TriadOS system toolkit")]
struct Cli { #[command(subcommand)] command: Command }
#[derive(Subcommand)]
enum Command {
    Info,
    System { #[command(subcommand)] command: SystemCommand },
    Apps { #[command(subcommand)] command: AppsCommand },
    Config,
    Services { #[command(subcommand)] command: ServiceCommand },
    Packages { #[command(subcommand)] command: PackageCommand },
    Core { #[command(subcommand)] command: CoreCommand },
    Logs { #[arg(short, long, default_value_t = 50)] lines: usize },
    Doctor,
    Image,
}
#[derive(Subcommand)]
enum SystemCommand { Status, Hardware }
#[derive(Subcommand)]
enum AppsCommand { List, Search { query: String }, Inspect { path: String }, Launch { desktop_file: String } }
#[derive(Subcommand)]
enum ServiceCommand { List, Status { name: String } }
#[derive(Subcommand)]
enum PackageCommand { Update, Search { query: String }, Install { package: String }, Remove { package: String } }
#[derive(Subcommand)]
enum CoreCommand { Serve, Status }

#[tokio::main]
async fn main() {
    if let Err(error) = run(Cli::parse()).await {
        eprintln!("triad: {error:#}");
        std::process::exit(1);
    }
}
async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Info => system::info(),
        Command::System { command } => match command { SystemCommand::Status => system::status(), SystemCommand::Hardware => system::hardware() },
        Command::Apps { command } => match command {
            AppsCommand::List => apps::list(),
            AppsCommand::Search { query } => apps::search(&query),
            AppsCommand::Inspect { path } => apps::inspect(&path),
            AppsCommand::Launch { desktop_file } => apps::launch_desktop(&desktop_file),
        },
        Command::Config => config::show(),
        Command::Services { command } => match command { ServiceCommand::List => services::list(), ServiceCommand::Status { name } => services::status(&name) },
        Command::Packages { command } => match command {
            PackageCommand::Update => packages::update(),
            PackageCommand::Search { query } => packages::search(&query),
            PackageCommand::Install { package } => packages::install(&package),
            PackageCommand::Remove { package } => packages::remove(&package),
        },
        Command::Core { command } => match command { CoreCommand::Serve => daemon::serve().await, CoreCommand::Status => daemon::status() },
        Command::Logs { lines } => logs::show(lines),
        Command::Doctor => diagnostics::doctor(),
        Command::Image => packages::build_image(),
    }
}
