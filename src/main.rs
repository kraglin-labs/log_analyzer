use log_analyzer::analyze_reader;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "log_analyzer".into());

    let Some(path) = args.next() else {
        eprintln!("Usage: {program} <log-file>");
        return ExitCode::from(2);
    };

    if args.next().is_some() {
        eprintln!("Usage: {program} <log-file>");
        return ExitCode::from(2);
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not open {path}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let stats = match analyze_reader(BufReader::new(file)) {
        Ok(stats) => stats,
        Err(err) => {
            eprintln!("Could not read {path}: {err}");
            return ExitCode::FAILURE;
        }
    };

    println!("Log analysis: {path}");
    println!("Total lines:     {}", stats.total_lines);
    println!("TRACE:           {}", stats.trace);
    println!("DEBUG:           {}", stats.debug);
    println!("INFO:            {}", stats.info);
    println!("WARN:            {}", stats.warn);
    println!("ERROR:           {}", stats.error);
    println!("FATAL:           {}", stats.fatal);
    println!("Other:           {}", stats.other);
    println!("Malformed JSON:  {}", stats.malformed_json);

    let mut errors: Vec<_> = stats.error_messages.iter().collect();
    errors.sort_by(|(message_a, count_a), (message_b, count_b)| {
        count_b.cmp(count_a).then_with(|| message_a.cmp(message_b))
    });

    println!("\nTop error messages:");
    if errors.is_empty() {
        println!("No errors found.");
    } else {
        for (message, count) in errors.into_iter().take(5) {
            println!("{count:>5}  {message}");
        }
    }

    ExitCode::SUCCESS
}

/*
Neptune square mars, neptune square venus
My client wants to know why she gets no p*nis
Neptune square mars, neptune square venus
I told her that is why ans she told me I'm a genius
*/
