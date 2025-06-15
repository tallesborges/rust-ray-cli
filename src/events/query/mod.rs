use anyhow::Result;
use serde_json::Value;
use crate::events::base::{EventEntry, EventProcessor, extract_timestamp, extract_origin_info};

pub struct QueryProcessor;

impl EventProcessor for QueryProcessor {
    fn process(&self, payload: &Value) -> Result<EventEntry> {
        let mut entry = EventEntry {
            timestamp: extract_timestamp(payload),
            label: "Query".to_string(),
            description: String::new(),
            content: String::new(),
            content_type: "markdown".to_string(),
        };

        if let Some(content) = payload.get("content") {
            let sql = content
                .get("sql")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let connection = content
                .get("connection_name")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let time = content
                .get("time")
                .and_then(|v| v.as_f64())
                .unwrap_or_default();

            // Extract the SQL operation type (SELECT, INSERT, UPDATE, etc.)
            let operation_type = if let Some(first_word) = sql.trim().split_whitespace().next() {
                first_word.to_uppercase()
            } else {
                "SQL".to_string()
            };

            // Generate a descriptive label
            entry.label = format!("Query: {}", operation_type);

            // Generate a more informative description from the SQL
            let description_sql = if sql.len() > 50 {
                format!("{}...", &sql[..50].trim())
            } else {
                sql.trim().to_string()
            };
            entry.description = format!("{} ({}ms)", description_sql, time);

            // Create a rich markdown presentation
            let mut markdown = String::from("## SQL Query\n\n");

            // Add basic query information
            markdown.push_str(&format!("**Operation:** {}\n\n", operation_type));
            if !connection.is_empty() {
                markdown.push_str(&format!("**Connection:** {}\n\n", connection));
            }

            // Format execution time with appropriate units
            let time_display = if time < 1.0 {
                format!("{:.3} ms", time)
            } else if time < 1000.0 {
                format!("{:.2} ms", time)
            } else {
                format!("{:.2} s", time / 1000.0)
            };

            markdown.push_str(&format!("**Execution Time:** {}\n\n", time_display));

            // Add SQL in a code block with syntax highlighting
            markdown.push_str("### Query\n\n```sql\n");
            markdown.push_str(sql);
            markdown.push_str("\n```\n");

            // Add source information if available
            if let Some(origin) = extract_origin_info(payload) {
                markdown.push_str(&format!("\n**Source:** {}\n", origin));
            }

            entry.content = markdown;
        }

        Ok(entry)
    }

    fn display_name(&self) -> &'static str {
        "SQL Query"
    }
}