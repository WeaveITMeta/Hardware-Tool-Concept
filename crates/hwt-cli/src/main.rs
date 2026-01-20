//! Hardware Tool CLI
//!
//! Command-line interface for Hardware Tool.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

mod commands;

/// Hardware Tool - One Hardware Tool That Does It All
#[derive(Parser)]
#[command(name = "hwt")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Hardware domain
        #[arg(short, long, default_value = "pcb")]
        domain: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Open a project
    Open {
        /// Project file path
        path: String,

        /// Open in live edit mode (for external formats)
        #[arg(long)]
        live: bool,
    },

    /// Import from external format
    Import {
        /// Source format (kicad, altium, eagle, tscircuit)
        format: String,

        /// Input file path
        input: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Export to external format
    Export {
        /// Target format (kicad, gerber, step, gdsii, circuit-json)
        format: String,

        /// Output path
        output: String,
    },

    /// Run design rule check
    Drc {
        /// Project file path
        #[arg(default_value = ".")]
        path: String,

        /// Output report file
        #[arg(short, long)]
        report: Option<String>,
    },

    /// Run benchmarks
    Benchmark {
        /// Benchmark suite (startup, render, drc, full)
        #[arg(default_value = "full")]
        suite: String,

        /// Number of iterations
        #[arg(short, long, default_value = "10")]
        iterations: u32,
    },

    /// Data model operations
    Data {
        #[command(subcommand)]
        command: DataCommands,
    },

    /// Show version and system info
    Info,
}

#[derive(Subcommand)]
enum DataCommands {
    /// Validate design data
    Validate {
        /// Design file path
        path: String,

        /// Strict validation
        #[arg(long)]
        strict: bool,
    },

    /// Migrate to new schema version
    Migrate {
        /// Design file path
        path: String,

        /// Target version
        #[arg(long, default_value = "latest")]
        to: String,
    },

    /// Convert between formats
    Convert {
        /// Input file path
        input: String,

        /// Output format (json, binary)
        #[arg(long)]
        to: String,
    },
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    if cli.verbose {
        tracing::info!("Verbose mode enabled");
    }

    match cli.command {
        Commands::New { name, domain, output } => {
            commands::new_project(&name, &domain, output.as_deref())?;
        }
        Commands::Open { path, live } => {
            commands::open_project(&path, live)?;
        }
        Commands::Import { format, input, output } => {
            commands::import_project(&format, &input, output.as_deref())?;
        }
        Commands::Export { format, output } => {
            commands::export_project(&format, &output)?;
        }
        Commands::Drc { path, report } => {
            commands::run_drc(&path, report.as_deref())?;
        }
        Commands::Benchmark { suite, iterations } => {
            commands::run_benchmark(&suite, iterations)?;
        }
        Commands::Data { command } => match command {
            DataCommands::Validate { path, strict } => {
                commands::validate_data(&path, strict)?;
            }
            DataCommands::Migrate { path, to } => {
                commands::migrate_data(&path, &to)?;
            }
            DataCommands::Convert { input, to } => {
                commands::convert_data(&input, &to)?;
            }
        },
        Commands::Info => {
            commands::show_info()?;
        }
    }

    Ok(())
}
