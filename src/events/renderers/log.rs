use crate::events::types::LogEvent;

pub fn render_log_markdown(log_event: &LogEvent) -> String {
    let mut markdown = String::from("## Log Entry\n\n");

    // Add log level if not default
    if log_event.level != "Info" {
        markdown.push_str(&format!("**Level:** {}\n\n", log_event.level));
    }

    // Add the main log message in a code block
    markdown.push_str("### Message\n\n```json\n");
    markdown.push_str(&log_event.message);
    markdown.push_str("\n```\n");

    // Add context if available
    if let Some(context) = &log_event.context {
        markdown.push_str("\n### Context\n\n```json\n");
        markdown.push_str(&serde_json::to_string_pretty(context).unwrap_or_default());
        markdown.push_str("\n```\n");
    }

    markdown
}

pub fn get_log_label(_log_event: &LogEvent) -> String {
    "Log".to_string()
}

pub fn get_log_description(log_event: &LogEvent) -> String {
    let description = if log_event.message.len() > 100 {
        format!("{}...", &log_event.message[..97])
    } else {
        log_event.message.clone()
    };
    
    // Clean up any JSON formatting for the description
    description.replace('\n', " ").replace("  ", " ").trim().to_string()
}