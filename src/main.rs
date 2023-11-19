mod logging;
mod build;
mod git;

use clap::{Parser, Subcommand};
use logging::*;

use std::time::SystemTime;
use std::path::PathBuf;
use std::process;
use std::env;

const FAILURE: i32 = 1;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, short, action)]
    silent: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install { url: Option<String> },
    Update { url: Option<String> },
}

fn main() {
    let args = Args::parse();

    let start = SystemTime::now();

    match &args.command {
        Commands::Install { url } | Commands::Update { url } => {
            let repo = if let Some(url) = url {
                info(&format!("installing package from `{url}`"));

                match git::clone(&url) {
                    Ok(repo) => repo,
                    Err(err) => {
                        error(&format!("failed to clone `{url}`: {}", err.to_string()));
                        process::exit(FAILURE);
                    },
                }
            } else {
                env::current_dir().unwrap_or(PathBuf::from("Unknown")).display().to_string()
            };

            info(&format!("building package `{repo}`"));

            let builder = match args.command {
                Commands::Install { .. } => {
                    build::Options::new()
                        .configure(true)
                        .build(true)
                        .dependencies(true)

                },
                Commands::Update { .. } => {
                    build::Options::new()
                        .build(true)
                        .dependencies(true)

                },
            };

            if let Err(err) = builder.install(&repo) {
                error(&format!("failed to build package `{repo}`: {}", err.to_string()));
            } else {
                info(&format!("package installed in {} seconds", start.elapsed().unwrap_or_default().as_secs_f64()));
            }
        },
    }
}


