use std::io::Cursor;

use log_analyzer::analyze_reader;

#[test]
fn analyzes_a_mixed_log_stream() {
    let input = concat!(
        "[INFO] Started\n",
        "[WARN] Slow response\n",
        "[ERROR] Database unavailable\n",
    );

    let stats = analyze_reader(Cursor::new(input)).unwrap();

    assert_eq!(stats.total_lines, 3);
    assert_eq!(stats.info, 1);
    assert_eq!(stats.warn, 1);
    assert_eq!(stats.error, 1);
    assert_eq!(stats.other, 0);
}
