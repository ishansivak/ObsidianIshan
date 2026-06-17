use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Synth { config } => {
            println!("Running synthesis with config: {}", config);
            // TODO: Map config to CommandConfig and execute
        }
        Commands::Pnr { config } => {
            println!("Running P&R with config: {}", config);
            // TODO: Map config to CommandConfig and execute
        }
        Commands::Load { config } => {
            println!("Running load with config: {}", config);
            // TODO: Map config to CommandConfig and execute
        }
    }
}
