use crate::event_storage::EventStorage;
use std::io::{self, stdout, Stdout};
use std::sync::Arc;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use shared::EventEntry;
use crate::tui_markdown; // Added import

struct TuiState {
    events: Vec<EventEntry>,
    selected_log_index: Option<usize>,
}

impl TuiState {
    fn new() -> Self {
        TuiState {
            events: Vec::new(),
            selected_log_index: None,
        }
    }

    fn update_events(&mut self, new_events: Vec<EventEntry>) {
        self.events = new_events;
        if self.events.is_empty() {
            self.selected_log_index = None;
        } else if self.selected_log_index.is_none()
            || self.selected_log_index.unwrap_or(0) >= self.events.len()
        {
            self.selected_log_index = Some(0);
        }
    }

    fn select_next(&mut self) {
        if self.events.is_empty() {
            return;
        }
        let new_index = match self.selected_log_index {
            Some(current_selected) => {
                if current_selected >= self.events.len() - 1 {
                    0 // Wrap to top
                } else {
                    current_selected + 1
                }
            }
            None => 0, // Select first if nothing is selected
        };
        self.selected_log_index = Some(new_index);
    }

    fn select_previous(&mut self) {
        if self.events.is_empty() {
            return;
        }
        let new_index = match self.selected_log_index {
            Some(current_selected) => {
                if current_selected == 0 {
                    self.events.len() - 1 // Wrap to bottom
                } else {
                    current_selected - 1
                }
            }
            None => self.events.len() - 1, // Select last if nothing is selected
        };
        self.selected_log_index = Some(new_index);
    }
}

pub fn run_tui_app(event_storage: Arc<EventStorage>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Ensure terminal is restored on panic or early exit
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal(); // Ignore errors during cleanup
        original_hook(panic_info);
    }));

    let result = tui_loop(&mut terminal, event_storage);

    // Restore terminal
    restore_terminal()?;

    result
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::EventEntry; // Make sure EventEntry is public in shared lib or adjust path

    fn create_dummy_event(id: &str) -> EventEntry {
        EventEntry {
            id: id.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            label: format!("label_{}", id),
            content: format!("content for {}", id),
            content_type: "text".to_string(),
            source_ip: Some("127.0.0.1".to_string()),
        }
    }

    #[test]
    fn test_tui_state_new() {
        let state = TuiState::new();
        assert!(state.events.is_empty());
        assert_eq!(state.selected_log_index, None);
    }

    #[test]
    fn test_tui_state_update_events() {
        let mut state = TuiState::new();

        // Test with empty vector
        state.update_events(Vec::new());
        assert!(state.events.is_empty());
        assert_eq!(state.selected_log_index, None);

        // Test with non-empty vector
        let events1 = vec![create_dummy_event("1"), create_dummy_event("2")];
        state.update_events(events1.clone());
        assert_eq!(state.events.len(), 2);
        assert_eq!(state.selected_log_index, Some(0));

        // Test updating with new events when an index was already selected
        state.selected_log_index = Some(1);
        let events2 = vec![create_dummy_event("3")];
        state.update_events(events2.clone());
        assert_eq!(state.events.len(), 1);
        assert_eq!(state.selected_log_index, Some(0)); // Resets to 0 for new list

        // Test updating with empty list when an index was selected
        state.selected_log_index = Some(0); // From previous state
        state.update_events(Vec::new());
        assert!(state.events.is_empty());
        assert_eq!(state.selected_log_index, None);

        // Test updating when selected_log_index is out of bounds
        state.update_events(vec![create_dummy_event("a"), create_dummy_event("b")]);
        state.selected_log_index = Some(5); // Out of bounds
        state.update_events(vec![create_dummy_event("c"), create_dummy_event("d"), create_dummy_event("e")]);
        assert_eq!(state.selected_log_index, Some(0));
    }

    #[test]
    fn test_tui_state_select_next() {
        let mut state = TuiState::new();

        // Test with no events
        state.select_next();
        assert_eq!(state.selected_log_index, None);

        // Test with one event
        let event1 = vec![create_dummy_event("1")];
        state.update_events(event1); // selected_log_index becomes Some(0)
        state.select_next();
        assert_eq!(state.selected_log_index, Some(0)); // Wraps to 0

        // Test with multiple events
        let events_multi = vec![create_dummy_event("1"), create_dummy_event("2"), create_dummy_event("3")];
        state.update_events(events_multi); // selected_log_index becomes Some(0)

        state.select_next(); // Selects 1
        assert_eq!(state.selected_log_index, Some(1));

        state.select_next(); // Selects 2
        assert_eq!(state.selected_log_index, Some(2));

        state.select_next(); // Wraps to 0
        assert_eq!(state.selected_log_index, Some(0));
        
        // Test select_next when nothing is selected (e.g. after events were empty)
        state.update_events(Vec::new());
        state.update_events(vec![create_dummy_event("a"), create_dummy_event("b")]); // index is Some(0)
        state.selected_log_index = None; // Manually set to None
        state.select_next();
        assert_eq!(state.selected_log_index, Some(0)); // Should select first item
    }

    #[test]
    fn test_tui_state_select_previous() {
        let mut state = TuiState::new();

        // Test with no events
        state.select_previous();
        assert_eq!(state.selected_log_index, None);

        // Test with one event
        let event1 = vec![create_dummy_event("1")];
        state.update_events(event1); // selected_log_index becomes Some(0)
        state.select_previous();
        assert_eq!(state.selected_log_index, Some(0)); // Wraps to 0 (which is also events.len() - 1)

        // Test with multiple events
        let events_multi = vec![create_dummy_event("1"), create_dummy_event("2"), create_dummy_event("3")];
        state.update_events(events_multi.clone()); // selected_log_index becomes Some(0)

        state.select_previous(); // Wraps to last (index 2)
        assert_eq!(state.selected_log_index, Some(2));

        state.select_previous(); // Selects 1
        assert_eq!(state.selected_log_index, Some(1));

        state.select_previous(); // Selects 0
        assert_eq!(state.selected_log_index, Some(0));

        // Test select_previous when nothing is selected
        state.update_events(Vec::new());
        state.update_events(events_multi); // index is Some(0)
        state.selected_log_index = None; // Manually set to None
        state.select_previous();
        assert_eq!(state.selected_log_index, Some(2)); // Should select last item
    }
}

fn tui_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    event_storage: Arc<EventStorage>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_state = TuiState::new();
    let mut list_state = ListState::default(); // For controlling the list selection

    loop {
        // Fetch logs and update state
        let fetched_events = event_storage.get_all_events();
        app_state.update_events(fetched_events);

        // Update ListState's selected index based on TuiState
        list_state.select(app_state.selected_log_index);

        terminal.draw(|frame| {
            // Layout
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(frame.size());
            let list_pane = chunks[0];
            let detail_pane = chunks[1];

            // Log List (Left Pane)
            let list_items: Vec<ListItem> = app_state
                .events
                .iter()
                .map(|event| {
                    ListItem::new(format!(
                        "ID: {} | {}",
                        event.id,
                        event.label
                    ))
                })
                .collect();

            let log_list_widget = List::new(list_items)
                .block(Block::default().title("Logs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray) // More prominent highlight
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            frame.render_stateful_widget(log_list_widget, list_pane, &mut list_state);

            // Log Detail (Right Pane)
            let detail_block = Block::default().title("Log Detail").borders(Borders::ALL);
            if let Some(selected_index) = app_state.selected_log_index {
                if let Some(selected_event) = app_state.events.get(selected_index) {
                    // Parse markdown content
                    let parsed_lines = tui_markdown::parse_markdown(&selected_event.content);
                    let paragraph = Paragraph::new(parsed_lines) // Use parsed lines
                        .block(detail_block)
                        .wrap(ratatui::widgets::Wrap { trim: false }); // Keep wrap for now
                    frame.render_widget(paragraph, detail_pane);
                } else {
                    // Should not happen if state is consistent
                    let placeholder = Paragraph::new(vec![Line::from("Error: Selected event not found.")])
                        .block(detail_block);
                    frame.render_widget(placeholder, detail_pane);
                }
            } else {
                let placeholder = Paragraph::new(vec![Line::from("No log selected or no logs available.")])
                    .block(detail_block);
                frame.render_widget(placeholder, detail_pane);
            }
        })?;

        // Event handling (exit on 'q', navigation placeholder)
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if let Some(current_selected) = app_state.selected_log_index {
                            if current_selected > 0 {
                                app_state.selected_log_index = Some(current_selected - 1);
                            }
                        } else if !app_state.events.is_empty() {
                             app_state.selected_log_index = Some(app_state.events.len() -1); // Wrap to bottom
                        }
                    }
                    KeyCode::Down => {
                        if let Some(current_selected) = app_state.selected_log_index {
                            if current_selected < app_state.events.len() - 1 {
                                app_state.selected_log_index = Some(current_selected + 1);
                            }
                        } else if !app_state.events.is_empty() {
                            app_state.selected_log_index = Some(0); // Wrap to top
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
