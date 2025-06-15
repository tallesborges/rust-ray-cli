use crate::events::types::ApplicationLogEvent;

pub fn render_application_log_markdown(app_log_event: &ApplicationLogEvent) -> String {
    let mut markdown = String::from("## Application Log\n\n");

    // Add log level if not default
    if app_log_event.level != "Info" {
        markdown.push_str(&format!("**Level:** {}\n\n", app_log_event.level));
    }

    // Add channel if available
    if let Some(channel) = &app_log_event.channel {
        markdown.push_str(&format!("**Channel:** {}\n\n", channel));
    }

    // Add log content in a code block
    markdown.push_str("### Log Content\n\n```\n");
    markdown.push_str(&app_log_event.message);
    markdown.push_str("\n```\n");

    // Add context if available
    if let Some(context) = &app_log_event.context {
        markdown.push_str("\n### Context\n\n```json\n");
        markdown.push_str(&serde_json::to_string_pretty(context).unwrap_or_default());
        markdown.push_str("\n```\n");
    }

    markdown
}

pub fn get_application_log_label(_app_log_event: &ApplicationLogEvent) -> String {
    "Application Log".to_string()
}

pub fn get_application_log_description(app_log_event: &ApplicationLogEvent) -> String {
    if !app_log_event.message.is_empty() {
        if app_log_event.message.len() > 50 {
            format!("{}...", &app_log_event.message[..50].trim())
        } else {
            app_log_event.message.clone()
        }
    } else {
        "Empty log".to_string()
    }
}