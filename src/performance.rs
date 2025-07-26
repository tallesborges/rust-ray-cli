use std::time::{Duration, Instant};
use crate::event_storage::EventStorage;
use crate::events::EventEntry;
use std::sync::Arc;
use serde_json::json;

pub struct PerformanceTracker {
    pub filter_times: Vec<Duration>,
    pub render_times: Vec<Duration>,
    pub cache_hits: u32,
    pub cache_misses: u32,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            filter_times: Vec::new(),
            render_times: Vec::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    pub fn record_filter_time(&mut self, duration: Duration) {
        self.filter_times.push(duration);
    }

    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    pub fn average_filter_time(&self) -> Option<Duration> {
        if self.filter_times.is_empty() {
            None
        } else {
            let total: Duration = self.filter_times.iter().sum();
            Some(total / self.filter_times.len() as u32)
        }
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    pub fn performance_report(&self) -> String {
        format!(
            "Performance Report:\n\
             - Filter operations: {}\n\
             - Average filter time: {:?}\n\
             - Cache hit rate: {:.2}%\n\
             - Cache hits: {}\n\
             - Cache misses: {}",
            self.filter_times.len(),
            self.average_filter_time().unwrap_or(Duration::ZERO),
            self.cache_hit_rate() * 100.0,
            self.cache_hits,
            self.cache_misses
        )
    }
}

// Benchmark utilities for measuring performance improvements
pub fn benchmark_filtering_performance(storage: &Arc<EventStorage>, iterations: usize) -> PerformanceTracker {
    let mut tracker = PerformanceTracker::new();
    
    // Generate test events
    for i in 0..1000 {
        let event = json!({
            "type": "test",
            "content": {
                "label": format!("Test Event {}", i),
                "description": format!("Performance test event number {}", i),
                "event_type": if i % 3 == 0 { "cache" } else if i % 3 == 1 { "http" } else { "log" },
                "timestamp": "2025-01-26T20:00:00Z"
            }
        });
        storage.add_event(&event);
    }

    // Benchmark filtering operations
    for _ in 0..iterations {
        let start = Instant::now();
        let _filtered = storage.get_events();
        let duration = start.elapsed();
        tracker.record_filter_time(duration);
    }

    tracker
}

// Memory usage estimation
pub fn estimate_memory_usage(events: &[EventEntry]) -> usize {
    events.iter().map(|event| {
        std::mem::size_of::<EventEntry>() +
        event.label.len() +
        event.description.len() +
        event.content_type.len() +
        event.event_type.len() +
        event.timestamp.len() +
        event.raw_payload.to_string().len()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_tracker() {
        let mut tracker = PerformanceTracker::new();
        tracker.record_filter_time(Duration::from_millis(10));
        tracker.record_filter_time(Duration::from_millis(20));
        tracker.record_cache_hit();
        tracker.record_cache_miss();

        assert_eq!(tracker.filter_times.len(), 2);
        assert_eq!(tracker.average_filter_time(), Some(Duration::from_millis(15)));
        assert_eq!(tracker.cache_hit_rate(), 0.5);
    }

    #[test]
    fn test_memory_estimation() {
        let event = EventEntry {
            timestamp: "2025-01-26".to_string(),
            label: "Test".to_string(),
            description: "Test description".to_string(),
            content_type: "test".to_string(),
            event_type: "test".to_string(),
            raw_payload: json!({"test": "data"}),
        };

        let memory_usage = estimate_memory_usage(&[event]);
        assert!(memory_usage > 0);
    }
}