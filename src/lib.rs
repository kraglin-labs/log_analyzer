use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LogStats {
    pub total_lines: usize,
    pub info: usize,
    pub warn: usize,
    pub error: usize,
    pub other: usize,
    pub error_messages: HashMap<String, usize>,
}

pub fn analyze_reader<R: BufRead>(reader: R) -> io::Result<LogStats> {
    let mut stats = LogStats::default();

    for line_result in reader.lines() {
        let line = line_result?;
        stats.total_lines += 1;

        let level = line
            .split(|c: char| !c.is_ascii_alphanumeric())
            .find(|token| {
                matches!(
                    token.to_ascii_uppercase().as_str(),
                    "INFO" | "WARN" | "WARNING" | "ERROR"
                )
            })
            .map(str::to_ascii_uppercase);

        match level.as_deref() {
            Some("INFO") => stats.info += 1,
            Some("WARN" | "WARNING") => stats.warn += 1,
            Some("ERROR") => {
                stats.error += 1;

                if let Some(message) = extract_error_message(&line) {
                    *stats.error_messages.entry(message).or_insert(0) += 1;
                }
            }
            _ => stats.other += 1,
        }
    }

    Ok(stats)
}

fn extract_error_message(line: &str) -> Option<String> {
    let upper = line.to_ascii_uppercase();
    let position = upper.find("ERROR")?;
    let message = line[position + "ERROR".len()..]
        .trim_start_matches(|c: char| c.is_whitespace() || matches!(c, ':' | '-' | ']' | '|'))
        .trim();

    if message.is_empty() {
        None
    } else {
        Some(message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn counts_log_levels() {
        let input = concat!(
            "[INFO] Server started\n",
            "[WARN] Slow request\n",
            "[ERROR] Connection refused\n",
            "Unclassified message\n",
            "[WARNING] Disk space low\n",
        );

        let stats = analyze_reader(Cursor::new(input)).unwrap();

        assert_eq!(stats.total_lines, 5);
        assert_eq!(stats.info, 1);
        assert_eq!(stats.warn, 2);
        assert_eq!(stats.error, 1);
        assert_eq!(stats.other, 1);
    }

    #[test]
    fn counts_repeated_error_messages() {
        let input = concat!(
            "[ERROR] Connection refused\n",
            "[ERROR] Connection refused\n",
            "[ERROR] Request timed out\n",
        );

        let stats = analyze_reader(Cursor::new(input)).unwrap();

        assert_eq!(stats.error_messages["Connection refused"], 2);
        assert_eq!(stats.error_messages["Request timed out"], 1);
    }

    #[test]
    fn handles_empty_input() {
        let stats = analyze_reader(Cursor::new("")).unwrap();

        assert_eq!(stats, LogStats::default());
    }

    #[test]
    fn recognizes_lowercase_levels() {
        let stats = analyze_reader(Cursor::new("info: started\nerror: failed\n")).unwrap();

        assert_eq!(stats.info, 1);
        assert_eq!(stats.error, 1);
    }
}
