use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub event_type: String,  // "progress", "log", "error", "complete"
    pub section: Option<u32>,
    pub total_sections: Option<u32>,
    pub message: String,
    pub timestamp: u64,
}

/// Parse a line of script output to detect section progress
pub fn parse_line(line: &str) -> Option<ExecutionEvent> {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Detect section headers like "1/13 —" or "2/13 —"
    if let Some(caps) = regex::Regex::new(r"(\d+)/13\s*[—–-]")
        .ok()?
        .captures(line)
    {
        let section: u32 = caps.get(1)?.as_str().parse().ok()?;
        return Some(ExecutionEvent {
            event_type: "progress".into(),
            section: Some(section),
            total_sections: Some(13),
            message: line.to_string(),
            timestamp: ts,
        });
    }

    // Detect completion
    if line.contains("GHOSTTY ULTIMATE — Ayu Dark") && line.contains("✓") {
        return Some(ExecutionEvent {
            event_type: "complete".into(),
            section: Some(13),
            total_sections: Some(13),
            message: line.to_string(),
            timestamp: ts,
        });
    }

    // Detect errors
    if line.contains("✗") {
        return Some(ExecutionEvent {
            event_type: "error".into(),
            section: None,
            total_sections: None,
            message: line.to_string(),
            timestamp: ts,
        });
    }

    None
}
