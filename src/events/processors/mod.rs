pub mod application_log;
pub mod cache;
pub mod exception;
pub mod http;
pub mod log;
pub mod query;
// pub mod table; // Removed - was part of anti-pattern dispatcher

pub use application_log::process_application_log_event;
pub use exception::process_exception_event;
pub use log::process_log_event;
pub use query::process_query_event;
// pub use table::process_table_event; // Removed - was part of anti-pattern dispatcher
