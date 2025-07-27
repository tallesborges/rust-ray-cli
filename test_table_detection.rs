// Quick test script to verify table event detection
use serde_json::json;

fn main() {
    // Test HTTP table event
    let http_table_event = json!({
        "type": "table",
        "content": {
            "label": "Http",
            "values": {
                "url": "https://api.example.com/test",
                "method": "GET",
                "status": 200
            }
        }
    });

    // Test Cache table event 
    let cache_table_event = json!({
        "type": "table", 
        "content": {
            "label": "Cache",
            "values": {
                "operation": "Hit",
                "key": "user:123",
                "value": "cached_data"
            }
        }
    });

    println!("Testing table event detection...");
    println!("HTTP table event: {:?}", http_table_event);
    println!("Cache table event: {:?}", cache_table_event);
}