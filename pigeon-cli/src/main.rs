mod serve;
mod mock_handlers;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pigeon")]
#[command(about = "The Pigeon RPC CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
     Serve {
        #[arg(short, long, default_value = "examples/helloworld/pigeon.yaml")]
        file: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();

    match cli.command{
        Commands::Serve { file } => {
             let registry = mock_handlers::register_all();
            serve::run_server(&file, registry).await?;
        }
    }

    Ok(())
}