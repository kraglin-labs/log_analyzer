use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use log_analyzer::{analyze_reader, LogStats};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,

    #[arg(short = 'n', long, default_value_t = 5)]
    top: usize,
    
    #[arg(short = 'l', long, value_name = "LEVEL")]
    min_level: Option<Severity>,

    #[arg(long)]
    json: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Severity {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let stats = match cli.path.as_deref() {
        Some(path) if path.as_os_str() != "-" => {
            let file = File::open(path)
                .with_context(|| format!("could not open {}", path.display()))?;
            analyze_reader(BufReader::new(file))
                .with_context(|| format!("could not read {}", path.display()))?
        }
        _ => analyze_reader(io::stdin().lock()).context("could not read stdin")?,
    };

    let top = top_errors(&stats, cli.top);

    if cli.json {
        print_json(&stats, &top);
    } else {
        print_text(&cli, &stats, &top);
    }

    Ok(())
}

fn top_errors<'a>(stats: &'a LogStats, n: usize) -> Vec<(&'a String, &'a usize)> {
    let mut errors: Vec<_> = stats.error_messages.iter().collect();
    errors.sort_by(|(m1, c1), (m2, c2)| c2.cmp(c1).then_with(|| m1.cmp(m2)));
    errors.truncate(n);
    errors
}

fn print_text(cli: &Cli, stats: &LogStats, top: &[(&String, &usize)]) {
    let source = cli
        .path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "stdin".into());

    println!("Log analysis: {source}");
    println!("Total lines:     {}", stats.total_lines);

    let rows = [
        ("TRACE", stats.trace, Severity::Trace),
        ("DEBUG", stats.debug, Severity::Debug),
        ("INFO", stats.info, Severity::Info),
        ("WARN", stats.warn, Severity::Warn),
        ("ERROR", stats.error, Severity::Error),
        ("FATAL", stats.fatal, Severity::Fatal),
    ];
    for (name, count, severity) in rows {
        if cli.min_level.is_some_and(|min| severity < min) {
            continue;
        }
        println!("{name:<6} {count}");
    }
    println!("Other:           {}", stats.other);
    println!("Malformed JSON:  {}", stats.malformed_json);

    println!("\nTop error messages:");
    if top.is_empty() {
        println!("No errors found.");
    } else {
        for (message, count) in top {
            println!("{count:>5}  {message}");
        }
    }
}

fn print_json(stats: &LogStats, top: &[(&String, &usize)]) {
    let output = json!({
        "total_lines": stats.total_lines,
        "levels": {
            "TRACE": stats.trace,
            "DEBUG": stats.debug,
            "INFO": stats.info,
            "WARN": stats.warn,
            "ERROR": stats.error,
            "FATAL": stats.fatal,
        },
        "other": stats.other,
        "malformed_json": stats.malformed_json,
        "top_errors": top
            .iter()
            .map(|(message, count)| json!({ "message": message, "count": count }))
            .collect::<Vec<_>>(),
    });
    println!("{output}");
}
