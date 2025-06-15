use crate::events::types::ExceptionEvent;

pub fn render_exception_markdown(exception_event: &ExceptionEvent) -> String {
    let mut markdown = format!("## {}\n\n", exception_event.class);

    if !exception_event.message.is_empty() {
        markdown.push_str(&format!("**Error:** {}\n\n", exception_event.message));
    }

    // Add file and line information
    if !exception_event.file.is_empty() {
        markdown.push_str(&format!(
            "**Location:** {}:{}\n\n",
            exception_event.file, exception_event.line
        ));
    }

    // Add stack trace if available
    if !exception_event.stack_trace.is_empty() {
        markdown.push_str("### Stack Trace\n\n");
        for (index, frame) in exception_event.stack_trace.iter().enumerate() {
            markdown.push_str(&format!("{}. ", index + 1));
            
            if let Some(class) = &frame.class {
                markdown.push_str(&format!("{}::", class));
            }
            
            markdown.push_str(&format!("{}() ", frame.function));
            markdown.push_str(&format!("at {}:{}\n", frame.file, frame.line));
        }
        markdown.push_str("\n");
    }

    // Add context if available
    if let Some(context) = &exception_event.context {
        markdown.push_str("### Context\n\n```json\n");
        markdown.push_str(&serde_json::to_string_pretty(context).unwrap_or_default());
        markdown.push_str("\n```\n");
    }

    markdown
}

pub fn get_exception_label(_exception_event: &ExceptionEvent) -> String {
    "Exception".to_string()
}

pub fn get_exception_description(exception_event: &ExceptionEvent) -> String {
    let description = if !exception_event.message.is_empty() {
        format!("{}: {}", exception_event.class, exception_event.message)
    } else {
        exception_event.class.clone()
    };

    // Truncate long descriptions
    if description.len() > 100 {
        format!("{}...", &description[..97])
    } else {
        description
    }
}