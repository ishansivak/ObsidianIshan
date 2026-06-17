use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "kiara")]
#[command(about = "Rust-native FPGA design suite", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run synthesis
    Synth {
        #[arg(short, long)]
        config: String,
    },
    /// Run placement and routing
    Pnr {
        #[arg(short, long)]
        config: String,
    },
    /// Load bitstream to FPGA
    Load {
        #[arg(short, long)]
        config: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct CommandConfig {
    tool_name: String,
    executable_path: String,
    arguments: Vec<String>,
    environment: std::collections::HashMap<String, String>,
    working_directory: String,
}

fn load_config(path: &str) -> Result<CommandConfig, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let config: CommandConfig = serde_json::from_str(&content)?;
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Synth { config } => {
            let cmd_config = load_config(config)?;
            println!("Running synthesis with config: {:?}", cmd_config);
            // TODO: Execute using ToolchainCommand
        }
        Commands::Pnr { config } => {
            let cmd_config = load_config(config)?;
            println!("Running P&R with config: {:?}", cmd_config);
            // TODO: Execute using ToolchainCommand
        }
        Commands::Load { config } => {
            let cmd_config = load_config(config)?;
            println!("Running load with config: {:?}", cmd_config);
            // TODO: Execute using ToolchainCommand
        }
    }
    Ok(())
}
