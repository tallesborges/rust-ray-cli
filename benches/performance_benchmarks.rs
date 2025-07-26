use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{Duration, Instant};

// Import the application modules we need to benchmark
use rust_ray_cli::events::EventEntry;
use rust_ray_cli::event_storage::EventStorage;

fn create_sample_event(index: usize) -> Value {
    json!({
        "content": {
            "label": "Http",
            "values": {
                "url": format!("https://api.example.com/endpoint/{}", index),
                "method": "GET",
                "status": 200,
                "duration": format!("{}ms", index % 500 + 50),
                "size": format!("{}kb", index % 100 + 10),
                "headers": {
                    "user-agent": "rust-ray-cli/1.0",
                    "content-type": "application/json"
                },
                "response_body": format!("{{\"data\": \"sample response {}\", \"count\": {}}}", index, index * 2)
            }
        },
        "timestamp": format!("2024-{:02}-{:02} {:02}:{:02}:{:02}", 
            (index % 12) + 1, 
            (index % 28) + 1, 
            (index % 24), 
            (index % 60), 
            (index % 60)
        ),
        "origin": {
            "file": format!("/app/src/api/endpoint_{}.rs", index % 10),
            "line_number": index % 100 + 1,
            "hostname": "api-server-01"
        }
    })
}

fn create_large_cache_event(index: usize) -> Value {
    json!({
        "content": {
            "label": "Cache",
            "values": {
                "operation": if index % 3 == 0 { "Hit" } else if index % 3 == 1 { "Missed" } else { "Key written" },
                "key": format!("user:profile:{}", index),
                "size": format!("{}kb", index % 500 + 10),
                "ttl": format!("{}s", index % 3600 + 300),
                "data": format!("{{\"user_id\": {}, \"profile\": {{\"name\": \"User {}\", \"email\": \"user{}@example.com\", \"settings\": {{\"theme\": \"dark\", \"notifications\": true}}}}}}", index, index, index)
            }
        },
        "timestamp": format!("2024-{:02}-{:02} {:02}:{:02}:{:02}", 
            (index % 12) + 1, 
            (index % 28) + 1, 
            (index % 24), 
            (index % 60), 
            (index % 60)
        ),
        "origin": {
            "file": "/app/src/cache/redis.rs",
            "line_number": index % 200 + 1,
            "hostname": "cache-server-01"
        }
    })
}

fn create_complex_table_event(index: usize) -> Value {
    json!({
        "content": {
            "label": "Database",
            "values": {
                "query": format!("SELECT * FROM users WHERE active = true AND created_at > '{}' LIMIT {}", 
                    format!("2024-{:02}-{:02}", (index % 12) + 1, (index % 28) + 1), 
                    index % 100 + 10
                ),
                "duration": format!("{}ms", index % 1000 + 50),
                "rows_affected": index % 1000,
                "database": "production",
                "table": "users",
                "operation": if index % 4 == 0 { "SELECT" } else if index % 4 == 1 { "INSERT" } else if index % 4 == 2 { "UPDATE" } else { "DELETE" },
                "transaction_id": format!("tx_{}", index),
                "connection_pool": format!("pool_{}", index % 10),
                "execution_plan": format!("Index Scan on users_active_idx (cost=0.43..{}.{} rows={} width={})", 
                    index % 100, index % 100, index % 1000, index % 200)
            }
        },
        "timestamp": format!("2024-{:02}-{:02} {:02}:{:02}:{:02}", 
            (index % 12) + 1, 
            (index % 28) + 1, 
            (index % 24), 
            (index % 60), 
            (index % 60)
        ),
        "origin": {
            "file": "/app/src/database/queries.rs",
            "line_number": index % 500 + 1,
            "hostname": "db-server-01"
        }
    })
}

fn benchmark_event_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_processing");
    
    // Test different payload sizes
    for size in [10, 100, 500, 1000, 2000].iter() {
        group.benchmark_with_input(
            BenchmarkId::new("http_events", size),
            size,
            |b, &size| {
                let events: Vec<Value> = (0..size).map(create_sample_event).collect();
                b.iter(|| {
                    for event in &events {
                        let _ = black_box(rust_ray_cli::events::table::process(event));
                    }
                });
            },
        );

        group.benchmark_with_input(
            BenchmarkId::new("cache_events", size),
            size,
            |b, &size| {
                let events: Vec<Value> = (0..size).map(create_large_cache_event).collect();
                b.iter(|| {
                    for event in &events {
                        let _ = black_box(rust_ray_cli::events::table::process(event));
                    }
                });
            },
        );

        group.benchmark_with_input(
            BenchmarkId::new("complex_table_events", size),
            size,
            |b, &size| {
                let events: Vec<Value> = (0..size).map(create_complex_table_event).collect();
                b.iter(|| {
                    for event in &events {
                        let _ = black_box(rust_ray_cli::events::table::process(event));
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_event_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_storage");
    
    for size in [100, 500, 1000, 2000, 5000].iter() {
        group.benchmark_with_input(
            BenchmarkId::new("storage_operations", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let storage = Arc::new(EventStorage::new());
                    let events: Vec<EventEntry> = (0..size)
                        .map(|i| {
                            let payload = create_sample_event(i);
                            rust_ray_cli::events::table::process(&payload).unwrap()
                        })
                        .collect();
                    
                    // Measure insertion performance
                    let start = Instant::now();
                    for event in events {
                        storage.add_event(event);
                    }
                    let duration = start.elapsed();
                    black_box(duration);
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_json_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_processing");
    
    // Test JSON serialization performance with size limits
    for size in [1000, 5000, 10000, 50000, 100000].iter() {
        group.benchmark_with_input(
            BenchmarkId::new("json_serialization_with_limits", size),
            size,
            |b, &size| {
                let large_json = json!({
                    "data": (0..*size).map(|i| format!("item_{}", i)).collect::<Vec<_>>(),
                    "metadata": {
                        "count": size,
                        "generated_at": "2024-07-26T12:00:00Z",
                        "version": "1.0"
                    }
                });
                
                b.iter(|| {
                    // Test the optimized JSON processing with size limits
                    let json_str = serde_json::to_string_pretty(&large_json).unwrap();
                    let result = if json_str.len() > 10000 {
                        format!("{}... [truncated {} chars]", 
                            &json_str[..1000], 
                            json_str.len() - 1000)
                    } else {
                        json_str
                    };
                    black_box(result);
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    
    group.bench_function("arc_based_filtering", |b| {
        let storage = Arc::new(EventStorage::new());
        
        // Add many events
        for i in 0..5000 {
            let payload = create_sample_event(i);
            let event = rust_ray_cli::events::table::process(&payload).unwrap();
            storage.add_event(event);
        }
        
        b.iter(|| {
            // Test the Arc-based filtering that avoids cloning
            let events = storage.get_filtered_events_cached(
                &std::collections::HashSet::new(),
                ""
            );
            black_box(events);
        });
    });
    
    group.finish();
}

fn benchmark_ui_virtualization(c: &mut Criterion) {
    let mut group = c.benchmark_group("ui_performance");
    
    // Simulate rendering performance with large datasets
    for event_count in [100, 500, 1000, 2000, 5000].iter() {
        group.benchmark_with_input(
            BenchmarkId::new("virtual_rendering_simulation", event_count),
            event_count,
            |b, &event_count| {
                let events: Vec<EventEntry> = (0..event_count)
                    .map(|i| {
                        let payload = create_sample_event(i);
                        rust_ray_cli::events::table::process(&payload).unwrap()
                    })
                    .collect();
                
                b.iter(|| {
                    // Simulate viewport-based rendering (only render visible items)
                    let viewport_start = 0;
                    let viewport_size = 20; // Typical viewport shows ~20 items
                    let viewport_end = std::cmp::min(viewport_start + viewport_size, events.len());
                    
                    let visible_events = &events[viewport_start..viewport_end];
                    
                    // Simulate optimized rendering operations
                    for (index, event) in visible_events.iter().enumerate() {
                        // Simulate the optimized label truncation
                        let display_label = if event.label.len() > 50 {
                            format!("{}...", &event.label[..47])
                        } else {
                            event.label.clone()
                        };
                        
                        // Simulate the optimized description truncation
                        let display_desc = if event.description.len() > 80 {
                            format!("{}...", &event.description[..77])
                        } else {
                            event.description.clone()
                        };
                        
                        black_box((index, display_label, display_desc));
                    }
                });
            },
        );
    }
    
    group.finish();
}

// FPS simulation benchmark - measures how fast we can process events in a frame
fn benchmark_fps_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("fps_simulation");
    
    // Target: 60 FPS = 16.67ms per frame
    let target_frame_time = Duration::from_millis(16);
    
    for event_count in [100, 500, 1000, 2000, 5000, 10000].iter() {
        group.benchmark_with_input(
            BenchmarkId::new("frame_processing", event_count),
            event_count,
            |b, &event_count| {
                let storage = Arc::new(EventStorage::new());
                
                // Pre-populate with events
                for i in 0..event_count {
                    let payload = create_sample_event(i);
                    let event = rust_ray_cli::events::table::process(&payload).unwrap();
                    storage.add_event(event);
                }
                
                b.iter(|| {
                    let frame_start = Instant::now();
                    
                    // Simulate a frame's worth of operations
                    let events = storage.get_filtered_events_cached(
                        &std::collections::HashSet::new(),
                        ""
                    );
                    
                    // Simulate viewport rendering (20 visible items)
                    let visible_count = std::cmp::min(20, events.len());
                    for i in 0..visible_count {
                        if let Some(event) = events.get(i) {
                            // Simulate optimized rendering
                            let _ = black_box(&event.label);
                            let _ = black_box(&event.description);
                            let _ = black_box(&event.timestamp);
                        }
                    }
                    
                    let frame_time = frame_start.elapsed();
                    black_box(frame_time);
                    
                    // Check if we can maintain 60 FPS
                    assert!(frame_time < target_frame_time, 
                        "Frame time {}ms exceeds target {}ms for {} events", 
                        frame_time.as_millis(), 
                        target_frame_time.as_millis(),
                        event_count
                    );
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_event_processing,
    benchmark_event_storage,
    benchmark_json_processing,
    benchmark_memory_usage,
    benchmark_ui_virtualization,
    benchmark_fps_simulation
);
criterion_main!(benches);