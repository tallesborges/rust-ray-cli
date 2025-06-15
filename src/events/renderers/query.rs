use crate::events::types::QueryEvent;

pub fn render_query_markdown(query_event: &QueryEvent) -> String {
    let mut markdown = String::from("## SQL Query\n\n");

    // Extract the SQL operation type (SELECT, INSERT, UPDATE, etc.)
    let operation_type = if let Some(first_word) = query_event.sql.trim().split_whitespace().next() {
        first_word.to_uppercase()
    } else {
        "SQL".to_string()
    };

    // Add basic query information
    markdown.push_str(&format!("**Operation:** {}\n\n", operation_type));

    if let Some(connection) = &query_event.connection_name {
        if !connection.is_empty() {
            markdown.push_str(&format!("**Connection:** {}\n\n", connection));
        }
    }

    // Format execution time with appropriate units
    if let Some(time) = query_event.duration_ms {
        let time_display = if time < 1.0 {
            format!("{:.3} ms", time)
        } else if time < 1000.0 {
            format!("{:.2} ms", time)
        } else {
            format!("{:.2} s", time / 1000.0)
        };
        markdown.push_str(&format!("**Execution Time:** {}\n\n", time_display));
    }

    if let Some(affected_rows) = query_event.affected_rows {
        markdown.push_str(&format!("**Affected Rows:** {}\n\n", affected_rows));
    }

    // Add SQL in a code block with syntax highlighting
    markdown.push_str("### Query\n\n```sql\n");
    markdown.push_str(&query_event.sql);
    markdown.push_str("\n```\n");

    // Add bindings if available
    if !query_event.bindings.is_empty() {
        markdown.push_str("\n### Bindings\n\n```json\n");
        markdown.push_str(&serde_json::to_string_pretty(&query_event.bindings).unwrap_or_default());
        markdown.push_str("\n```\n");
    }

    markdown
}

pub fn get_query_label(query_event: &QueryEvent) -> String {
    let operation_type = if let Some(first_word) = query_event.sql.trim().split_whitespace().next() {
        first_word.to_uppercase()
    } else {
        "SQL".to_string()
    };
    format!("Query: {}", operation_type)
}

pub fn get_query_description(query_event: &QueryEvent) -> String {
    let description_sql = if query_event.sql.len() > 50 {
        format!("{}...", &query_event.sql[..50].trim())
    } else {
        query_event.sql.trim().to_string()
    };
    
    if let Some(time) = query_event.duration_ms {
        format!("{} ({}ms)", description_sql, time)
    } else {
        description_sql
    }
}