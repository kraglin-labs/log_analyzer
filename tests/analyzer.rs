use log_analyzer::analyze_reader;
use std::io::Cursor;

#[test]
fn parses_plain_text_levels_without_matching_message_words() {
    let input = concat!(
        "[INFO] Server started\n",
        "[ERROR] Database unavailable\n",
        "INFO: Request completed\n",
        "ERROR: Database unavailable\n",
        "[WARN] Error count is increasing\n",
        "This message mentions ERROR but has no level prefix\n",
    );

    let stats = analyze_reader(Cursor::new(input)).unwrap();

    assert_eq!(stats.total_lines, 6);
    assert_eq!(stats.info, 2);
    assert_eq!(stats.error, 2);
    assert_eq!(stats.warn, 1);
    assert_eq!(stats.other, 1);
    assert_eq!(stats.error_messages["Database unavailable"], 2);
}

#[test]
fn parses_json_with_level_and_message_fields() {
    let input = concat!(
        "{\"level\":\"INFO\",\"message\":\"Started\"}\n",
        "{\"level\":\"ERROR\",\"message\":\"Connection refused\"}\n",
        "{\"level\":\"ERROR\",\"message\":\"Connection refused\"}\n",
        "{\"level\":\"WARNING\",\"message\":\"Slow query\"}\n",
    );

    let stats = analyze_reader(Cursor::new(input)).unwrap();

    assert_eq!(stats.total_lines, 4);
    assert_eq!(stats.info, 1);
    assert_eq!(stats.error, 2);
    assert_eq!(stats.warn, 1);
    assert_eq!(stats.error_messages["Connection refused"], 2);
}

#[test]
fn accepts_alternate_json_field_names_and_lowercase_levels() {
    let input = concat!(
        "{\"severity\":\"fatal\",\"msg\":\"Disk failure\"}\n",
        "{\"severity\":\"debug\",\"msg\":\"Cache refreshed\"}\n",
        "{\"severity\":\"trace\",\"msg\":\"Request received\"}\n",
    );

    let stats = analyze_reader(Cursor::new(input)).unwrap();

    assert_eq!(stats.fatal, 1);
    assert_eq!(stats.debug, 1);
    assert_eq!(stats.trace, 1);
    assert_eq!(stats.error_messages["Disk failure"], 1);
}

#[test]
fn handles_mixed_formats_and_malformed_json() {
    let input = concat!(
        "[INFO] Starting service\n",
        "{\"level\":\"ERROR\",\"message\":\"Request failed\"}\n",
        "{\"level\":\n",
        "{\"message\":\"Missing level\"}\n",
        "\n",
    );

    let stats = analyze_reader(Cursor::new(input)).unwrap();

    assert_eq!(stats.total_lines, 5);
    assert_eq!(stats.info, 1);
    assert_eq!(stats.error, 1);
    assert_eq!(stats.malformed_json, 1);
    assert_eq!(stats.other, 3);
}

#[test]
fn handles_empty_input() {
    let stats = analyze_reader(Cursor::new("")).unwrap();

    assert_eq!(stats.total_lines, 0);
    assert_eq!(stats.error, 0);
    assert_eq!(stats.other, 0);
    assert!(stats.error_messages.is_empty());
}
