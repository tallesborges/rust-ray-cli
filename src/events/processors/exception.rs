use crate::events::types::{ExceptionEvent, ProcessedEvent, StackFrame};
use anyhow::Result;
use serde_json::Value;

pub fn process_exception_event(content: &Value) -> Result<ProcessedEvent> {
    let class = content
        .get("class")
        .and_then(Value::as_str)
        .unwrap_or("Unknown Exception")
        .to_string();

    let message = content
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let file = content
        .get("file")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let line = content.get("line").and_then(Value::as_u64).unwrap_or(0);

    // Extract stack trace
    let mut stack_trace = Vec::new();
    if let Some(frames) = content.get("frames").and_then(Value::as_array) {
        for frame in frames {
            if let Some(frame_obj) = frame.as_object() {
                let frame_file = frame_obj
                    .get("file")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let frame_line = frame_obj.get("line").and_then(Value::as_u64).unwrap_or(0);
                let function = frame_obj
                    .get("function")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let class = frame_obj
                    .get("class")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());

                stack_trace.push(StackFrame {
                    file: frame_file,
                    line: frame_line,
                    function,
                    class,
                });
            }
        }
    }

    let context = content.get("context").cloned();

    Ok(ProcessedEvent::Exception(ExceptionEvent {
        class,
        message,
        file,
        line,
        stack_trace,
        context,
    }))
}
