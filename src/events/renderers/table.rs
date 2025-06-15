use crate::events::types::TableEvent;

pub fn render_table_markdown(table_event: &TableEvent) -> String {
    let mut markdown = format!("## {}\n\n", table_event.label);

    // Convert fields to a bulleted list for better readability
    for (key, value) in &table_event.data {
        if value.is_object() || value.is_array() {
            markdown.push_str(&format!(
                "### {}\n\n```json\n{}\n```\n\n",
                key,
                serde_json::to_string_pretty(value).unwrap_or_default()
            ));
        } else if let Some(val_str) = value.as_str() {
            markdown.push_str(&format!("- **{}:** {}\n", key, val_str));
        } else if let Some(val_num) = value.as_u64() {
            markdown.push_str(&format!("- **{}:** {}\n", key, val_num));
        } else if let Some(val_bool) = value.as_bool() {
            markdown.push_str(&format!("- **{}:** {}\n", key, val_bool));
        } else if value.is_null() {
            markdown.push_str(&format!("- **{}:** *null*\n", key));
        } else {
            markdown.push_str(&format!("- **{}:** {}\n", key, value));
        }
    }

    // If no structured data was found, fall back to raw JSON
    if table_event.data.is_empty() {
        markdown.push_str("*No data available*\n");
    }

    markdown
}

pub fn get_table_label(table_event: &TableEvent) -> String {
    table_event.label.clone()
}

pub fn get_table_description(table_event: &TableEvent) -> String {
    format!("{} data", table_event.label)
}
