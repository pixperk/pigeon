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
    Generate {
        #[arg(short, long, default_value = "examples/helloworld/pigeon.yaml")]
        file: String,
        #[arg(short, long, default_value = "generated_structs.rs")]
        output: String,
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
        Commands::Generate { file, output } => {
            let content = std::fs::read_to_string(&file)?;
            let schema: pigeon_core::schema::Schema = serde_yaml::from_str(&content)?;
            
            piegon_codegen::write_structs_to_file(&schema, &output)?;
            println!("Generated structs written to: {}", output);
        }
    }

    Ok(())
}