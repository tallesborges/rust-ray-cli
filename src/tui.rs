use crate::event_storage::EventStorage;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs, Wrap},
};
use shared::EventEntry;
use std::{
    error::Error,
    io,
    sync::Arc,
    time::{Duration, Instant},
};
use chrono::Local;
use ratatui::widgets::ListState;
use crate::event_storage::LogLevel;

pub enum AppTab {
    Events,
    Stats,
    Logs,
    Help,
}

pub struct TuiApp {
    event_storage: Arc<EventStorage>,
    selected_index: usize,
    should_quit: bool,
    active_tab: AppTab,
    show_help_popup: bool,
    selected_log_index: usize,
    content_scroll: usize,  // Scroll position for content area
}

impl TuiApp {
    pub fn new(event_storage: Arc<EventStorage>) -> Self {
        Self {
            event_storage,
            selected_index: 0,
            should_quit: false,
            active_tab: AppTab::Events,
            show_help_popup: false,
            selected_log_index: 0,
            content_scroll: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Log startup and server info
        self.event_storage.info("TUI", "Starting TUI interface");
        self.event_storage.info("Server", &self.event_storage.get_server_info());
        
        // Setup terminal
        enable_raw_mode().map_err(|e| {
            let err_msg = format!("Failed to enable raw mode: {}", e);
            self.event_storage.error("TUI", &err_msg);
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn Error>
        })?;
        
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(|e| {
                let err_msg = format!("Failed to enter alternate screen: {}", e);
                self.event_storage.error("TUI", &err_msg);
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn Error>
            })?;
            
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)
            .map_err(|e| {
                let err_msg = format!("Failed to create terminal: {}", e);
                self.event_storage.error("TUI", &err_msg);
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn Error>
            })?;
            
        self.event_storage.info("TUI", "Terminal setup complete");

        // Main application loop
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        // Main loop with better error handling
        let result = (|| -> Result<(), Box<dyn Error>> {
            while !self.should_quit {
                terminal.draw(|f| self.ui(f))?;

                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                match crossterm::event::poll(timeout) {
                    Ok(true) => {
                        match event::read() {
                            Ok(Event::Key(key)) => {
                                if key.kind == KeyEventKind::Press {
                                    self.handle_key_event(key.code, key.modifiers);
                                }
                            }
                            Ok(_) => {} // Ignore other events
                            Err(e) => {
                                let err_msg = format!("Error reading event: {}", e);
                                self.event_storage.error("TUI", &err_msg);
                            }
                        }
                    }
                    Ok(false) => {} // No event, continue
                    Err(e) => {
                        let err_msg = format!("Error polling for events: {}", e);
                        self.event_storage.error("TUI", &err_msg);
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    last_tick = Instant::now();
                }
            }
            Ok(())
        })();

        // Cleanup - attempt to restore terminal state even if the main loop failed
        self.event_storage.info("TUI", "Cleaning up terminal state");
        let mut cleanup_successful = true;
        
        if let Err(e) = disable_raw_mode() {
            let err_msg = format!("Failed to disable raw mode: {}", e);
            self.event_storage.error("TUI", &err_msg);
            cleanup_successful = false;
        }
        
        if let Err(e) = execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ) {
            let err_msg = format!("Failed to leave alternate screen: {}", e);
            self.event_storage.error("TUI", &err_msg);
            cleanup_successful = false;
        }
        
        if let Err(e) = terminal.show_cursor() {
            let err_msg = format!("Failed to show cursor: {}", e);
            self.event_storage.error("TUI", &err_msg);
            cleanup_successful = false;
        }

        // Return the main loop result, or cleanup error if main loop succeeded but cleanup failed
        if let Err(e) = result {
            Err(e)
        } else if !cleanup_successful {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to restore terminal state"
            )))
        } else {
            Ok(())
        }
    }

    // Handle key events with scrolling support
    fn handle_key_event(&mut self, key_code: KeyCode, modifiers: KeyModifiers) {
        if self.show_help_popup {
            if matches!(key_code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?')) {
                self.show_help_popup = false;
            }
            return;
        }

        match key_code {
            KeyCode::Char('q') => {
                self.event_storage.info("TUI", "Application exiting");
                self.should_quit = true;
            },
            KeyCode::Esc => {
                self.event_storage.info("TUI", "Application exiting");
                self.should_quit = true; // Allow Escape to quit as well
            },
            KeyCode::Char('c') => {
                // Regular 'c' key doesn't do anything special
                // Ctrl+C is handled separately
            }
            KeyCode::Up | KeyCode::Char('k') => {
                match self.active_tab {
                    AppTab::Events => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // Ctrl+k scrolls content up (faster with shift)
                            let scroll_amount = if modifiers.contains(KeyModifiers::SHIFT) { 10 } else { 1 };
                            if self.content_scroll > scroll_amount {
                                self.content_scroll -= scroll_amount;
                            } else {
                                self.content_scroll = 0;
                            }
                        } else {
                            // Regular up/k moves selection
                            if self.selected_index > 0 {
                                self.selected_index -= 1;
                                self.content_scroll = 0; // Reset scroll when changing selection
                            }
                        }
                    },
                    AppTab::Logs => {
                        if self.selected_log_index > 0 {
                            self.selected_log_index -= 1;
                        }
                    },
                    _ => {}
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                match self.active_tab {
                    AppTab::Events => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // Ctrl+j scrolls content down (faster with shift)
                            let scroll_amount = if modifiers.contains(KeyModifiers::SHIFT) { 10 } else { 1 };
                            self.content_scroll += scroll_amount;
                            // Upper limit checking happens during rendering
                        } else {
                            // Regular down/j moves selection
                            let events = self.event_storage.get_events();
                            if !events.is_empty() && self.selected_index < events.len() - 1 {
                                self.selected_index += 1;
                                self.content_scroll = 0; // Reset scroll when changing selection
                            }
                        }
                    },
                    AppTab::Logs => {
                        let app_logs = self.event_storage.get_app_logs();
                        let total_logs = app_logs.len();
                        
                        if total_logs > 0 && self.selected_log_index < total_logs - 1 {
                            self.selected_log_index += 1;
                        }
                    },
                    _ => {}
                }
            }
            KeyCode::Tab | KeyCode::Right => {
                self.active_tab = match self.active_tab {
                    AppTab::Events => AppTab::Stats,
                    AppTab::Stats => AppTab::Logs,
                    AppTab::Logs => AppTab::Help,
                    AppTab::Help => AppTab::Events,
                };
                self.content_scroll = 0; // Reset scroll when changing tabs
            }
            KeyCode::BackTab | KeyCode::Left => {
                self.active_tab = match self.active_tab {
                    AppTab::Events => AppTab::Help,
                    AppTab::Stats => AppTab::Events,
                    AppTab::Logs => AppTab::Stats,
                    AppTab::Help => AppTab::Logs,
                };
                self.content_scroll = 0; // Reset scroll when changing tabs
            }
            KeyCode::Char('1') => {
                self.active_tab = AppTab::Events;
                self.content_scroll = 0; // Reset scroll when changing tabs
            },
            KeyCode::Char('2') => {
                self.active_tab = AppTab::Stats;
                self.content_scroll = 0; // Reset scroll when changing tabs
            },
            KeyCode::Char('3') => {
                self.active_tab = AppTab::Logs;
                self.content_scroll = 0; // Reset scroll when changing tabs
            },
            KeyCode::Char('4') => {
                self.active_tab = AppTab::Help;
                self.content_scroll = 0; // Reset scroll when changing tabs
            },
            KeyCode::Char('C') => {
                match self.active_tab {
                    AppTab::Events => {
                        self.event_storage.info("TUI", "Cleared all events");
                        self.event_storage.clear_events();
                        self.selected_index = 0;
                        self.content_scroll = 0;
                    },
                    AppTab::Logs => {
                        self.event_storage.info("TUI", "Cleared all logs");
                        self.event_storage.clear_logs();
                        self.selected_log_index = 0;
                    },
                    _ => {
                        // Clear both events and logs when on other tabs
                        self.event_storage.info("TUI", "Cleared all events and logs");
                        self.event_storage.clear_events();
                        self.event_storage.clear_logs();
                        self.selected_index = 0;
                        self.selected_log_index = 0;
                        self.content_scroll = 0;
                    }
                }
            }
            KeyCode::Char('?') => {
                self.show_help_popup = true;
            }
            _ => {}
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(frame.size());

        // Render tabs
        let titles = vec!["Events (1)", "Stats (2)", "Logs (3)", "Help (4)"];
        let tabs = Tabs::new(titles)
            .block(Block::default().title("RustRay TUI").borders(Borders::ALL))
            .select(match self.active_tab {
                AppTab::Events => 0,
                AppTab::Stats => 1,
                AppTab::Logs => 2,
                AppTab::Help => 3,
            })
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
        frame.render_widget(tabs, chunks[0]);

        // Render content based on selected tab
        match self.active_tab {
            AppTab::Events => self.render_events_tab(frame, chunks[1]),
            AppTab::Stats => self.render_stats_tab(frame, chunks[1]),
            AppTab::Logs => self.render_logs_tab(frame, chunks[1]),
            AppTab::Help => self.render_help_tab(frame, chunks[1]),
        }

        // Render help popup if requested
        if self.show_help_popup {
            self.render_help_popup(frame);
        }
    }

    fn render_events_tab(&mut self, frame: &mut Frame, area: Rect) {
        // Reset content scroll if we're in this tab
        if matches!(self.active_tab, AppTab::Events) {
            // Keep the content_scroll value
        }
        let events = self.event_storage.get_events();

        // Split the area into two columns
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        // Create event list with style based on event type
        let events_list: Vec<ListItem> = events
            .iter()
            .map(|e| {
                let style = self.get_style_for_event(e);
                let content = Line::from(vec![
                    Span::styled(format!("{} ", e.timestamp), style),
                    Span::styled(format!("[{}]", e.label), style.add_modifier(Modifier::BOLD)),
                ]);
                ListItem::new(content)
            })
            .collect();

        let events_widget = List::new(events_list)
            .block(Block::default()
                .title(format!("Events ({})", events.len()))
                .borders(Borders::ALL))
            .highlight_style(Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        // Render event list with selection
        let events_state = &ListState::default().with_selected(
            if events.is_empty() { None } else { Some(self.selected_index) }
        );
        frame.render_stateful_widget(events_widget, columns[0], &mut events_state.clone());

        // Render event details or help message
        if let Some(event) = events.get(self.selected_index) {
            let header = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(columns[1]);

            // Event metadata header
            let metadata = Table::new(vec![
                Row::new(vec![
                    Cell::from(format!("Type: {}", event.label)),
                    Cell::from(format!("Time: {}", event.timestamp)),
                ])
            ])
            .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
            .block(Block::default().borders(Borders::ALL).title("Metadata"));
            frame.render_widget(metadata, header[0]);

            // Event content with scrolling
            let content_lines: Vec<&str> = event.content.lines().collect();
            let content_height = header[1].height as usize - 2; // Account for borders
            
            // Calculate valid scroll position
            let max_scroll = if content_lines.len() > content_height {
                content_lines.len() - content_height
            } else {
                0
            };
            if self.content_scroll > max_scroll {
                self.content_scroll = max_scroll;
            }
            
            // Get the visible portion of content based on scroll position
            let visible_content = content_lines
                .iter()
                .skip(self.content_scroll)
                .take(content_height)
                .cloned()
                .collect::<Vec<&str>>()
                .join("\n");
            
            // Add scroll indicators for better visualization
            let scroll_indicator = if max_scroll > 0 {
                let position = (self.content_scroll as f64 / max_scroll as f64 * 100.0) as usize;
                let indicator = "▓".repeat(position / 10) + &"░".repeat(10 - position / 10);
                format!(" [{}] {}/{}", indicator, self.content_scroll, max_scroll)
            } else {
                String::new()
            };
            
            let details_widget = Paragraph::new(visible_content)
                .block(Block::default()
                    .title(format!("Content ({} format){}", &event.content_type, scroll_indicator))
                    .borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            frame.render_widget(details_widget, header[1]);
        } else {
            let help_text = "No events to display.\n\nPress 'q' to quit, 'C' to clear events.\nUse up/down arrows or j/k to navigate.";
            let help_widget = Paragraph::new(help_text)
                .block(Block::default().title("Help").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            frame.render_widget(help_widget, columns[1]);
        }
    }

    fn render_stats_tab(&mut self, frame: &mut Frame, area: Rect) {
        let events = self.event_storage.get_events();

        // Count events by type
        let mut event_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for event in &events {
            *event_counts.entry(event.label.clone()).or_insert(0) += 1;
        }

        // Create stats text
        let mut stats_text = format!("Total Events: {}\n\nEvent Types:\n", events.len());
        for (event_type, count) in event_counts.iter() {
            stats_text.push_str(&format!("- {}: {} events\n", event_type, count));
        }

        // Create the stats widget
        let stats_widget = Paragraph::new(stats_text)
            .block(Block::default().title("Statistics").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        frame.render_widget(stats_widget, area);
    }

    fn render_help_tab(&mut self, frame: &mut Frame, area: Rect) {
        // Get server information from storage
        let server_info = self.event_storage.get_server_info();
        
        let help_text = format!("\
RustRay TUI - Terminal User Interface

Navigation:
- Tab / Right Arrow: Next tab
- Shift+Tab / Left Arrow: Previous tab
- 1/2/3: Switch to specific tab
- Up/Down or j/k: Navigate through events
- q/Esc: Quit application
- C: Clear all events
- ?: Show help popup

Tab 1 - Events: View and explore all events
Tab 2 - Stats: View statistics about collected events
Tab 3 - Logs: View application and server logs
Tab 4 - Help: Display this help information

Status: {}
", if server_info.is_empty() { "Server status unknown" } else { &server_info });

        let help_widget = Paragraph::new(help_text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        frame.render_widget(help_widget, area);
    }

    fn render_help_popup(&mut self, frame: &mut Frame) {
        let area = frame.size();

        // Calculate popup size and position
        let popup_width = area.width.min(60);
        let popup_height = area.height.min(20);
        let popup_x = (area.width - popup_width) / 2;
        let popup_y = (area.height - popup_height) / 2;
        let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

        // Create a background clear
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            area,
        );

        // Create the popup
        let help_text = "\
KEYBOARD SHORTCUTS

Navigation:
- Tab/Right: Next tab
- Shift+Tab/Left: Previous tab
- 1/2/3/4: Switch to specific tab
- Up/Down or j/k: Navigate through events/logs

Actions:
- q/Esc: Quit application
- C: Clear events/logs (depends on current tab)
- ?: Toggle this help popup
- Ctrl+j/Ctrl+k: Scroll content down/up when viewing events
- Ctrl+Shift+j/k: Scroll content faster (10 lines at a time)

Press ? or ESC to close this popup
";

        let help_widget = Paragraph::new(help_text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .wrap(Wrap { trim: true });

        frame.render_widget(help_widget, popup_area);
    }

    fn render_logs_tab(&mut self, frame: &mut Frame, area: Rect) {
        // Get logs from storage
        let app_logs = self.event_storage.get_app_logs();
        
        // Create a list of logs with timestamps
        let mut logs = Vec::new();
        
        // Add all application logs with their timestamps and proper formatting
        // Reverse iteration to show newest logs at the top
        for log_entry in app_logs.iter().rev() {
            let style = match log_entry.level {
                LogLevel::Error => Style::default().fg(Color::Red),
                LogLevel::Warning => Style::default().fg(Color::Yellow),
                LogLevel::Info => Style::default().fg(Color::Green),
                LogLevel::Debug => Style::default().fg(Color::Blue),
            };
            
            let level_str = match log_entry.level {
                LogLevel::Error => "ERROR",
                LogLevel::Warning => "WARN",
                LogLevel::Info => "INFO",
                LogLevel::Debug => "DEBUG",
            };
            
            let content = Line::from(vec![
                Span::styled(format!("{} ", log_entry.timestamp), Style::default().fg(Color::Gray)),
                Span::styled(format!("[{}] [{}] ", level_str, log_entry.source), style.add_modifier(Modifier::BOLD)),
                Span::raw(&log_entry.message),
            ]);
            
            logs.push(ListItem::new(content));
        }
        
        let logs_widget = List::new(logs)
            .block(Block::default()
                .title(format!("Application Logs ({})", app_logs.len()))
                .borders(Borders::ALL))
            .highlight_style(Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        
        // If there are logs, show them with a selection
        if !app_logs.is_empty() {
            let mut logs_state = ListState::default();
            
            // Get total log count
            let total_logs = app_logs.len();
            
            // Keep selection in bounds - always start with the most recent log
            if self.selected_log_index >= total_logs {
                self.selected_log_index = 0; // Always select the newest log when new logs arrive
            }
            logs_state.select(Some(self.selected_log_index));
            
            frame.render_stateful_widget(logs_widget, area, &mut logs_state);
        } else {
            // If no logs, just render the widget without selection
            frame.render_widget(logs_widget, area);
        }
    }

    fn get_style_for_event(&mut self, event: &EventEntry) -> Style {
        match event.label.as_str() {
            "exception" => Style::default().fg(Color::Red),
            "query" => Style::default().fg(Color::Blue),
            "application_log" => Style::default().fg(Color::Yellow),
            "table" => Style::default().fg(Color::Green),
            "log" => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        }
    }
}
