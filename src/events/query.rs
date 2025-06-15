use crate::events::base::{extract_origin_info, extract_timestamp, EventEntry};
use crate::ui_components::{border_color, styled_card, text_monospace_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div, InteractiveText, StyledText};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "Query".to_string(),
        description: String::new(),
        content: String::new(),
        content_type: "markdown".to_string(),
        event_type: "query".to_string(),
        raw_payload: payload.clone(),
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

pub fn render_query_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    let content = entry
        .raw_payload
        .get("content")
        .cloned()
        .unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(render_query_header(&content))
        .child(render_query_metrics(&content))
        .child(render_sql_query(&content))
        .child(render_origin_info(entry))
}

fn render_query_header(content: &Value) -> Div {
    let sql = content.get("sql").and_then(|s| s.as_str()).unwrap_or("");
    let operation_type = if let Some(first_word) = sql.trim().split_whitespace().next() {
        first_word.to_uppercase()
    } else {
        "SQL".to_string()
    };

    let (icon, color) = match operation_type.as_str() {
        "SELECT" => ("🔍", gpui::rgb(0x3b82f6)), // Blue
        "INSERT" => ("➕", gpui::rgb(0x10b981)), // Green
        "UPDATE" => ("✏️", gpui::rgb(0xf59e0b)), // Yellow
        "DELETE" => ("🗑️", gpui::rgb(0xef4444)), // Red
        "CREATE" => ("🏗️", gpui::rgb(0x8b5cf6)), // Purple
        "DROP" => ("💥", gpui::rgb(0xef4444)),   // Red
        _ => ("💾", gpui::rgb(0x6b7280)),        // Gray
    };

    styled_card().p_4().child(
        div()
            .flex()
            .flex_row()
            .gap_3()
            .items_center()
            .child(div().text_2xl().child(icon))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(color)
                            .child(format!("{} Query", operation_type)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary_color())
                            .child("SQL Database Query"),
                    ),
            ),
    )
}

fn render_query_metrics(content: &Value) -> Div {
    let time = content.get("time").and_then(|t| t.as_f64()).unwrap_or(0.0);
    let connection = content
        .get("connection_name")
        .and_then(|c| c.as_str())
        .unwrap_or("default");

    let (time_display, time_color) = if time < 1.0 {
        (format!("{:.3} ms", time), gpui::rgb(0x10b981)) // Green - fast
    } else if time < 100.0 {
        (format!("{:.2} ms", time), gpui::rgb(0xf59e0b)) // Yellow - medium
    } else if time < 1000.0 {
        (format!("{:.1} ms", time), gpui::rgb(0xef4444)) // Red - slow
    } else {
        (format!("{:.2} s", time / 1000.0), gpui::rgb(0x7c2d12)) // Dark red - very slow
    };

    styled_card().p_4().child(
        div()
            .flex()
            .flex_row()
            .gap_6()
            .child(render_metric(
                "⏱️".to_string(),
                "Execution Time".to_string(),
                time_display,
                time_color,
            ))
            .child(render_metric(
                "🔗".to_string(),
                "Connection".to_string(),
                connection.to_string(),
                gpui::rgb(0x6b7280),
            ))
            .child(render_performance_indicator(time)),
    )
}

fn render_metric(icon: String, label: String, value: String, color: gpui::Rgba) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .items_center()
                .child(div().text_sm().child(icon))
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .child(label),
                ),
        )
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(color)
                .child(value),
        )
}

fn render_performance_indicator(time: f64) -> Div {
    let (label, color, bars) = if time < 1.0 {
        ("Excellent", gpui::rgb(0x10b981), 1)
    } else if time < 10.0 {
        ("Good", gpui::rgb(0x84cc16), 2)
    } else if time < 100.0 {
        ("Fair", gpui::rgb(0xf59e0b), 3)
    } else if time < 1000.0 {
        ("Slow", gpui::rgb(0xef4444), 4)
    } else {
        ("Very Slow", gpui::rgb(0x7c2d12), 5)
    };

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .items_center()
                .child(div().text_sm().child("📊"))
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .child("Performance"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(color)
                        .child(label),
                )
                .child(render_performance_bars(bars, color)),
        )
}

fn render_performance_bars(filled: usize, color: gpui::Rgba) -> Div {
    let mut container = div().flex().flex_row().gap_1();

    for i in 0..5 {
        container = container.child(div().w_2().h_1().rounded_sm().bg(if i < filled {
            color
        } else {
            gpui::rgb(0xe5e7eb)
        }));
    }

    container
}

fn render_sql_query(content: &Value) -> Div {
    let sql = content.get("sql").and_then(|s| s.as_str()).unwrap_or("");

    styled_card().p_4().child(
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(text_secondary_color())
                    .child("💾 SQL Query"),
            )
            .child(
                div()
                    .p_4()
                    .bg(gpui::rgb(0x1f2937)) // Dark background for code
                    .rounded_lg()
                    .border_1()
                    .border_color(border_color())
                    .max_h_64()
                    .overflow_hidden()
                    .child(render_highlighted_sql(sql)),
            ),
    )
}

fn render_highlighted_sql(sql: &str) -> Div {
    // Simple SQL syntax highlighting
    let highlighted = highlight_sql_keywords(sql);

    div()
        .font_family("monospace")
        .text_sm()
        .text_color(text_monospace_color())
        .child(InteractiveText::new(
            "sql-query",
            StyledText::new(highlighted),
        ))
}

fn highlight_sql_keywords(sql: &str) -> String {
    // This is a simple highlighting approach - in a real app you'd use a proper syntax highlighter
    let keywords = [
        "SELECT",
        "FROM",
        "WHERE",
        "JOIN",
        "INNER",
        "LEFT",
        "RIGHT",
        "OUTER",
        "INSERT",
        "UPDATE",
        "DELETE",
        "CREATE",
        "DROP",
        "ALTER",
        "TABLE",
        "INDEX",
        "PRIMARY",
        "KEY",
        "FOREIGN",
        "REFERENCES",
        "NOT",
        "NULL",
        "DEFAULT",
        "AUTO_INCREMENT",
        "UNIQUE",
        "CONSTRAINT",
        "ON",
        "AS",
        "GROUP",
        "BY",
        "ORDER",
        "HAVING",
        "LIMIT",
        "OFFSET",
        "DISTINCT",
        "AND",
        "OR",
        "IN",
        "LIKE",
        "BETWEEN",
        "IS",
        "EXISTS",
        "UNION",
        "VARCHAR",
        "INT",
        "BIGINT",
        "DECIMAL",
        "DATE",
        "DATETIME",
        "TEXT",
    ];

    let mut result = sql.to_string();

    for keyword in keywords {
        // Simple case-insensitive replacement
        // In a real implementation, you'd want proper tokenization
        let _pattern = format!(r"\b{}\b", keyword);
        result = result.replace(keyword, &format!("**{}**", keyword));
        result = result.replace(&keyword.to_lowercase(), &format!("**{}**", keyword));
    }

    result
}

fn render_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(|l| l.as_u64())
            .unwrap_or(0);
        let hostname = origin
            .get("hostname")
            .and_then(|h| h.as_str())
            .unwrap_or("");

        if !file.is_empty() {
            styled_card().p_3().child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_secondary_color())
                            .child("🔍 Source"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_monospace_color())
                            .child(format!("{}:{} on {}", file, line, hostname)),
                    ),
            )
        } else {
            div() // Empty div if no origin info
        }
    } else {
        div() // Empty div if no origin
    }
}
