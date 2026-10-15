use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Default)]
pub struct LogStats {
    pub total_lines: usize,
    pub trace: usize,
    pub debug: usize,
    pub info: usize,
    pub warn: usize,
    pub error: usize,
    pub fatal: usize,
    pub other: usize,
    pub malformed_json: usize,
    pub error_messages: HashMap<String, usize>,
}

#[derive(Debug, PartialEq, Eq)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Other,
}

#[derive(Debug)]
struct LogEntry {
    level: Level,
    message: String,
}

pub fn analyze_reader<R: BufRead>(reader: R) -> io::Result<LogStats> {
    let mut stats = LogStats::default();

    for line_result in reader.lines() {
        let line = line_result?;
        stats.total_lines += 1;

        let trimmed = line.trim();

        if trimmed.is_empty() {
            stats.other += 1;
            continue;
        }

        let entry = if trimmed.starts_with('{') {
            match parse_json_entry(trimmed) {
                Ok(entry) => entry,
                Err(()) => {
                    stats.malformed_json += 1;
                    stats.other += 1;
                    continue;
                }
            }
        } else {
            parse_text_entry(trimmed)
        };

        match entry.level {
            Level::Trace => stats.trace += 1,
            Level::Debug => stats.debug += 1,
            Level::Info => stats.info += 1,
            Level::Warn => stats.warn += 1,
            Level::Error => {
                stats.error += 1;
                *stats.error_messages.entry(entry.message).or_insert(0) += 1;
            }
            Level::Fatal => {
                stats.fatal += 1;
                *stats.error_messages.entry(entry.message).or_insert(0) += 1;
            }
            Level::Other => stats.other += 1,
        }
    }

    Ok(stats)
}

fn parse_json_entry(line: &str) -> Result<LogEntry, ()> {
    let value: Value = serde_json::from_str(line).map_err(|_| ())?;

    let object = value.as_object().ok_or(())?;

    let level = object
        .get("level")
        .or_else(|| object.get("severity"))
        .and_then(Value::as_str)
        .map(parse_level)
        .unwrap_or(Level::Other);

    let message = object
        .get("message")
        .or_else(|| object.get("msg"))
        .and_then(Value::as_str)
        .unwrap_or(line)
        .to_owned();

    Ok(LogEntry { level, message })
}

fn parse_text_entry(line: &str) -> LogEntry {
    let (level, message) = if line.starts_with('[') {
        match line.find(']') {
            Some(end) => {
                let candidate = &line[1..end];
                let rest = line[end + 1..].trim();
                if is_known_level(candidate) {
                    (parse_level(candidate), rest)
                } else {
                    (Level::Other, line)
                }
            }
            None => (Level::Other, line),
        }
    } else {
        let (first, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));

        let candidate = first.trim_end_matches(':');
        if is_known_level(candidate) {
            (parse_level(candidate), rest.trim())
        } else {
            (Level::Other, line)
        }
    };

    LogEntry {
        level,
        message: message.to_owned(),
    }
}

fn is_known_level(value: &str) -> bool {
    matches!(
        value.to_ascii_uppercase().as_str(),
        "TRACE" | "DEBUG" | "INFO" | "WARN" | "WARNING" | "ERROR" | "FATAL"
    )
}

fn parse_level(value: &str) -> Level {
    match value.to_ascii_uppercase().as_str() {
        "TRACE" => Level::Trace,
        "DEBUG" => Level::Debug,
        "INFO" => Level::Info,
        "WARN" | "WARNING" => Level::Warn,
        "ERROR" => Level::Error,
        "FATAL" => Level::Fatal,
        _ => Level::Other,
    }
}
