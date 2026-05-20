use anyhow::{Context, Result};
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use std::fs::File;
use std::path::PathBuf;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use venin_core::ToolRegistry;
use venin_parsers::browser::chromium_history::{
    write_browser_history_csv, write_browser_history_json, ChromiumHistoryParser,
};
use venin_timeline::{write_timeline_csv, write_timeline_json, TimelineBuilder, TimelineEvent};

#[derive(Debug, Parser)]
#[command(name = "venin")]
#[command(about = "CLI-first digital forensics toolkit for repeatable artefact parsing.")]
#[command(version)]
struct Cli {
    #[arg(short, long, global = true, action = ArgAction::Count)]
    verbose: u8,

    #[arg(long, global = true)]
    json_logs: bool,

    #[arg(long, global = true, default_value = "output")]
    output_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Inspect registered forensic tools and parsers.")]
    Registry {
        #[command(subcommand)]
        command: RegistryCommands,
    },
    #[command(about = "Parse forensic artefacts.")]
    Parse {
        #[command(subcommand)]
        command: ParseCommands,
    },
    #[command(about = "Build or transform normalized timelines.")]
    Timeline {
        #[command(subcommand)]
        command: TimelineCommands,
    },
    #[command(about = "Acquisition workflow entry points.")]
    Acquire {
        #[command(subcommand)]
        command: AcquireCommands,
    },
    #[command(about = "Mobile forensic workflow entry points.")]
    Mobile {
        #[command(subcommand)]
        command: MobileCommands,
    },
}

#[derive(Debug, Subcommand)]
enum RegistryCommands {
    List {
        #[arg(long)]
        json: bool,
    },
    Show {
        tool_id: String,
    },
}

#[derive(Debug, Subcommand)]
enum ParseCommands {
    #[command(
        name = "chrome-history",
        about = "Parse a Chromium-family History SQLite database."
    )]
    ChromeHistory(ChromeHistoryArgs),
}

#[derive(Debug, Args)]
struct ChromeHistoryArgs {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(long)]
    evidence_id: Option<String>,

    #[arg(long, value_enum, default_value = "both")]
    format: ExportFormat,
}

#[derive(Debug, Subcommand)]
enum TimelineCommands {
    Build {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(long, value_enum, default_value = "both")]
        format: ExportFormat,
    },
}

#[derive(Debug, Subcommand)]
enum AcquireCommands {
    Memory,
}

#[derive(Debug, Subcommand)]
enum MobileCommands {
    #[command(name = "ios-backup")]
    IosBackup {
        #[arg(short, long)]
        input: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ExportFormat {
    Json,
    Csv,
    Both,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose, cli.json_logs);

    let mut registry = ToolRegistry::new();
    venin_parsers::register_builtin_tools(&mut registry)?;

    match cli.command {
        Commands::Registry { command } => handle_registry(command, &registry),
        Commands::Parse { command } => handle_parse(command, cli.output_dir),
        Commands::Timeline { command } => handle_timeline(command, cli.output_dir),
        Commands::Acquire { command } => handle_acquire(command),
        Commands::Mobile { command } => handle_mobile(command),
    }
}

fn handle_registry(command: RegistryCommands, registry: &ToolRegistry) -> Result<()> {
    match command {
        RegistryCommands::List { json } => {
            if json {
                let tools: Vec<_> = registry.list().cloned().collect();
                println!("{}", serde_json::to_string_pretty(&tools)?);
            } else {
                for tool in registry.list() {
                    println!(
                        "{}\t{:?}\t{:?}\t{}",
                        tool.id, tool.kind, tool.status, tool.display_name
                    );
                }
            }
        }
        RegistryCommands::Show { tool_id } => {
            let tool = registry
                .get(&tool_id)
                .with_context(|| format!("unknown tool id: {tool_id}"))?;
            println!("{}", serde_json::to_string_pretty(tool)?);
        }
    }

    Ok(())
}

fn handle_parse(command: ParseCommands, output_dir: PathBuf) -> Result<()> {
    match command {
        ParseCommands::ChromeHistory(args) => {
            debug!(tool_id = "browser_history.chromium", "starting parser");
            let result = ChromiumHistoryParser::parse_path(&args.input, args.evidence_id)
                .with_context(|| "failed to parse Chromium History database")?;

            let parser_dir = output_dir.join("parsed").join("browser_history");
            let timeline_dir = output_dir.join("timelines");
            let mut builder = TimelineBuilder::new();
            builder.extend(result.timeline_events.clone());
            let timeline = builder.build();

            if matches!(args.format, ExportFormat::Json | ExportFormat::Both) {
                write_browser_history_json(parser_dir.join("chromium_history.json"), &result)?;
                write_timeline_json(
                    timeline_dir.join("chromium_history.timeline.json"),
                    &timeline,
                )?;
            }

            if matches!(args.format, ExportFormat::Csv | ExportFormat::Both) {
                write_browser_history_csv(
                    parser_dir.join("chromium_history.csv"),
                    &result.records,
                )?;
                write_timeline_csv(
                    timeline_dir.join("chromium_history.timeline.csv"),
                    &timeline,
                )?;
            }

            info!(
                tool_id = "browser_history.chromium",
                records = result.records.len(),
                timeline_events = timeline.len(),
                "parser completed"
            );
            println!(
                "Parsed {} URL records and {} timeline events into {}",
                result.records.len(),
                timeline.len(),
                output_dir.display()
            );
        }
    }

    Ok(())
}

fn handle_timeline(command: TimelineCommands, output_dir: PathBuf) -> Result<()> {
    match command {
        TimelineCommands::Build { input, format } => {
            let file = File::open(&input)
                .with_context(|| format!("failed to open timeline input {}", input.display()))?;
            let events: Vec<TimelineEvent> = serde_json::from_reader(file)
                .with_context(|| "timeline input must be a JSON array of TimelineEvent objects")?;

            let mut builder = TimelineBuilder::new();
            builder.extend(events);
            let timeline = builder.build();
            let timeline_dir = output_dir.join("timelines");

            if matches!(format, ExportFormat::Json | ExportFormat::Both) {
                write_timeline_json(timeline_dir.join("unified.timeline.json"), &timeline)?;
            }

            if matches!(format, ExportFormat::Csv | ExportFormat::Both) {
                write_timeline_csv(timeline_dir.join("unified.timeline.csv"), &timeline)?;
            }

            println!(
                "Built unified timeline with {} events into {}",
                timeline.len(),
                timeline_dir.display()
            );
        }
    }

    Ok(())
}

fn handle_acquire(command: AcquireCommands) -> Result<()> {
    match command {
        AcquireCommands::Memory => {
            println!("Memory acquisition is scaffolded. TODO: add platform-specific acquisition modules.");
        }
    }
    Ok(())
}

fn handle_mobile(command: MobileCommands) -> Result<()> {
    match command {
        MobileCommands::IosBackup { input } => {
            println!(
                "iOS backup workflow scaffold selected for {}. TODO: implement Manifest.db traversal.",
                input.display()
            );
        }
    }
    Ok(())
}

fn init_tracing(verbosity: u8, json_logs: bool) {
    let default_level = match verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    if json_logs {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .compact()
            .init();
    }
}
