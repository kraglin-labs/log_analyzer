use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::ExitCode;

use log_analyzer::analyze_reader;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "log_analyzer".into());

    let Some(path) = args.next() else {
        eprintln!("Usage: {program} <log-file>");
        return Err("missing log file argument".into());
    };

    if args.next().is_some() {
        eprintln!("Usage: {program} <log-file>");
        return Err("too many arguments".into());
    }

    let file = File::open(&path)?;
    let stats = analyze_reader(BufReader::new(file))?;

    println!("Log Analysis Summary");
    println!("oooooooonobodyoooooo");
    println!("File:         {path}");
    println!("Total lines:  {}", stats.total_lines);
    println!("INFO:         {}", stats.info);
    println!("WARN:         {}", stats.warn);
    println!("ERROR:        {}", stats.error);
    println!("OTHER:        {}", stats.other);

    let mut errors: Vec<_> = stats.error_messages.iter().collect();
    errors.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    if !errors.is_empty() {
        println!("\nTop error messages:");

        for (message, count) in errors.iter().take(5) {
            println!("- {message} ({count})");
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
