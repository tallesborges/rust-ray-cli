use std::fmt;
use std::str::FromStr;

/// Centralized event type enum to replace scattered string-based filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventType {
    Cache,
    Http,
    Log,
    Query,
    Exception,
    ApplicationLog,
    Table,
}

impl EventType {
    /// Get all available event types as a vec
    pub fn all() -> Vec<EventType> {
        vec![
            EventType::Cache,
            EventType::Http,
            EventType::Log,
            EventType::Query,
            EventType::Exception,
            EventType::ApplicationLog,
            EventType::Table,
        ]
    }

    /// Convert to string representation used in the UI
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Cache => "cache",
            EventType::Http => "http",
            EventType::Log => "log",
            EventType::Query => "query",
            EventType::Exception => "exception",
            EventType::ApplicationLog => "application_log",
            EventType::Table => "table",
        }
    }

    /// Get the display name for UI (prettier version)
    pub fn display_name(&self) -> &'static str {
        match self {
            EventType::Cache => "Cache",
            EventType::Http => "HTTP",
            EventType::Log => "Log",
            EventType::Query => "Query",
            EventType::Exception => "Exception",
            EventType::ApplicationLog => "Application Log",
            EventType::Table => "Table",
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cache" => Ok(EventType::Cache),
            "http" | "request" => Ok(EventType::Http), // Support both http and request
            "log" => Ok(EventType::Log),
            "query" | "executed_query" => Ok(EventType::Query), // Support both query variants
            "exception" => Ok(EventType::Exception),
            "application_log" => Ok(EventType::ApplicationLog),
            "table" => Ok(EventType::Table),
            _ => Err(format!("Unknown event type: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_from_str() {
        assert_eq!("cache".parse::<EventType>().unwrap(), EventType::Cache);
        assert_eq!("http".parse::<EventType>().unwrap(), EventType::Http);
        assert_eq!("request".parse::<EventType>().unwrap(), EventType::Http); // Legacy support
        assert_eq!("query".parse::<EventType>().unwrap(), EventType::Query);
        assert_eq!("executed_query".parse::<EventType>().unwrap(), EventType::Query); // Legacy support
    }

    #[test]
    fn test_event_type_display() {
        assert_eq!(EventType::Cache.to_string(), "cache");
        assert_eq!(EventType::Http.to_string(), "http");
        assert_eq!(EventType::ApplicationLog.display_name(), "Application Log");
    }

    #[test]
    fn test_all_event_types() {
        let all_types = EventType::all();
        assert_eq!(all_types.len(), 7);
        assert!(all_types.contains(&EventType::Cache));
        assert!(all_types.contains(&EventType::Http));
    }
}