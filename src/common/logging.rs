use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub trait Log {
    fn info(&mut self, msg: &str);
    fn warn(&mut self, msg: &str);
    fn ok_with_highlight(&mut self, msg: &str, highlight: &str);
    fn add_group(&mut self, title: &str, affected_count: usize);
    fn snapshot(&self) -> LogSnapshot;
}

#[derive(Default)]
pub struct MemoryLogger {
    lines: Vec<LogLine>,
    groups: Vec<GroupSummary>,
}

#[derive(Clone)]
pub struct LogLine {
    pub level: LogLevel,
    pub msg: String,
    pub highlight: Option<String>,
}

#[derive(Clone)]
pub struct GroupSummary {
    pub title: String,
    pub affected_count: usize,
}

#[derive(Clone)]
pub enum LogLevel {
    Info,
    Ok,
    Warn,
}

impl Log for MemoryLogger {
    fn info(&mut self, msg: &str) {
        self.lines.push(LogLine { level: LogLevel::Info, msg: msg.to_string(), highlight: None });
    }
    fn warn(&mut self, msg: &str) {
        self.lines.push(LogLine { level: LogLevel::Warn, msg: msg.to_string(), highlight: None });
    }
    fn ok_with_highlight(&mut self, msg: &str, highlight: &str) {
        self.lines.push(LogLine { level: LogLevel::Ok, msg: msg.to_string(), highlight: Some(highlight.to_string()) });
    }

    fn add_group(&mut self, title: &str, affected_count: usize) {
        self.groups.push(GroupSummary { title: title.to_string(), affected_count });
    }

    fn snapshot(&self) -> LogSnapshot {
        LogSnapshot {
            lines: self.lines.clone(),
            groups: self.groups.clone(),
        }
    }
}

pub struct LogSnapshot {
    pub lines: Vec<LogLine>,
    pub groups: Vec<GroupSummary>,
}

pub fn render_ui(snapshot: &LogSnapshot, err: Option<String>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(size);

        let title = Paragraph::new(Line::from(vec![
            Span::styled("mac-setup", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" — Setup Summary"),
        ]))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = snapshot
            .lines
            .iter()
            .map(|l| {
                let base_style = match l.level {
                    LogLevel::Info => Style::default(),
                    LogLevel::Ok => Style::default().fg(Color::Green),
                    LogLevel::Warn => Style::default().fg(Color::Yellow),
                };
                let mut spans: Vec<Span> = vec![Span::styled(l.msg.clone(), base_style)];
                if let Some(h) = &l.highlight {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(h.clone(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
                }
                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Steps").borders(Borders::ALL));
        f.render_widget(list, chunks[1]);

        let footer_text = if let Some(e) = err {
            format!("Error: {}", e)
        } else {
            let mut summary_parts: Vec<String> = Vec::new();
            for g in &snapshot.groups {
                summary_parts.push(format!("{}: {}", g.title, g.affected_count));
            }
            if summary_parts.is_empty() {
                "Summary: no changes".to_string()
            } else {
                format!("Summary — {}", summary_parts.join(" · "))
            }
        };
        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    })?;

    disable_raw_mode()?;
    Ok(())
}
