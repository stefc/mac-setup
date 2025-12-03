use std::env;
use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

mod detectors;
use detectors::{AppDetector, WezTermDetector, OhMyZshDetector, VSCodeDetector, YaziDetector};
mod common;
use common::replace_home_with_tilde;
mod symlinks;
use symlinks::{SymlinkCreator, ShellSymlinkCreator, SymlinkConfig, SetupResult};
mod configurators;
use configurators::{Configurator, ZshrcConfigurator, YaziConfigurator};

fn main() {
    let orchestrator = SetupOrchestrator::new(ShellSymlinkCreator);
    let mut logger = Logger::default();

    let res = orchestrator.run_with_logger(&mut logger);

    // Render a simple Ratatui UI summarizing the outcome
    if let Err(e) = render_ui(&logger, res.as_ref().err().map(|e| e.to_string())) {
        eprintln!("Failed to render UI: {}", e);
    }

    // Also print a plain-text summary to stdout for non-TUI contexts
    if logger.groups.is_empty() {
        // Fallback: no groups recorded
        println!("Summary: no changes");
    } else {
        let parts: Vec<String> = logger
            .groups
            .iter()
            .map(|g| format!("{}: {}", g.title, g.affected_count))
            .collect();
        println!("Summary — {}", parts.join(" · "));
    }

    if let Err(e) = res {
        eprintln!("Setup failed: {}", e);
        std::process::exit(1);
    }
}

/// Orchestrates setup tasks (Single Responsibility, Dependency Inversion)
struct SetupOrchestrator<C: SymlinkCreator> {
    symlink_creator: C,
}

impl<C: SymlinkCreator> SetupOrchestrator<C> {
    fn new(symlink_creator: C) -> Self {
        Self { symlink_creator }
    }

    fn run_with_logger(&self, logger: &mut Logger) -> SetupResult<()> {
        logger.info(format!("Current working directory: {}", current_working_directory()));
        logger.info(format!("Executable directory: {}", executable_directory()));
        self.run_configurators(logger)?;
        self.setup_symlinks(logger)?;

        Ok(())
    }

    fn run_configurators(&self, logger: &mut Logger) -> SetupResult<()> {
        let configurators: Vec<Box<dyn Configurator>> = vec![
            Box::new(YaziConfigurator),
            Box::new(ZshrcConfigurator::default()),
        ];
        let mut affected = 0usize;
        for configurator in configurators {
            if configurator.should_run() {
                logger.info(format!("{} configuration needed, configuring...", configurator.name()));
                configurator.configure()?;
                let files = configurator.affected_files();
                for file in files {
                    logger.ok_with_highlight("Configured successfully ->".to_string(), file);
                }
                affected += 1;
            } else {
                logger.info(format!("{} configuration not needed, skipping.", configurator.name()));
            }
        }

        logger.add_group("Configurators".to_string(), affected);

        Ok(())
    }

    fn setup_symlinks(&self, logger: &mut Logger) -> SetupResult<()> {
        // Get the executable directory and construct config path
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path.parent().expect("Failed to get executable directory");
        let config_dir = exe_dir.join("config");
        let config_dir_str = config_dir.display().to_string();

        let configs = vec![
            (
                Box::new(WezTermDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/.wezterm.lua", config_dir_str),
                    destination: "~/.wezterm.lua".to_string(),
                    installer_name: "WezTerm".to_string(),
                    success_message: "Symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(OhMyZshDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/stefc.zsh-theme", config_dir_str),
                    destination: "~/.oh-my-zsh/themes/stefc.zsh-theme".to_string(),
                    installer_name: "oh-my-zsh".to_string(),
                    success_message: "Theme symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(VSCodeDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/code.settings.json", config_dir_str),
                    destination: "~/Library/Application Support/Code/User/settings.json".to_string(),
                    installer_name: "Visual Studio Code".to_string(),
                    success_message: "Settings symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(YaziDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/yazi.theme.toml", config_dir_str),
                    destination: "~/.config/yazi/theme.toml".to_string(),
                    installer_name: "Yazi".to_string(),
                    success_message: "Theme symlink created successfully".to_string(),
                },
            ),
        ];

        let mut affected = 0usize;
        for (detector, config) in configs {
            if detector.is_installed() {
                logger.info(format!(
                    "{} is installed, creating symlink for {} config...",
                    detector.name(),
                    config.installer_name
                ));
                self.symlink_creator.create(&config)?;
                logger.ok_with_highlight(format!("{} ->", config.success_message), config.destination.clone());
                affected += 1;
            } else {
                logger.warn(format!(
                    "{} is not installed, skipping {} config symlink creation.",
                    detector.name(),
                    config.installer_name
                ));
            }
        }

        logger.add_group("Symlinks".to_string(), affected);

        Ok(())
    }
}

/// Print current working directory with tilde substitution
fn current_working_directory() -> String {
    let path = env::current_dir().expect("Failed to get current working directory");
    replace_home_with_tilde(path.display().to_string())
}

fn executable_directory() -> String {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    exe_path
        .parent()
        .map(|p| replace_home_with_tilde(p.display().to_string()))
        .unwrap_or_else(|| "<unknown>".to_string())
}

#[derive(Default)]
struct Logger {
    lines: Vec<LogLine>,
    groups: Vec<GroupSummary>,
}

#[derive(Clone)]
struct LogLine {
    level: LogLevel,
    msg: String,
    highlight: Option<String>,
}

#[derive(Clone)]
struct GroupSummary {
    title: String,
    affected_count: usize,
}

#[derive(Clone)]
enum LogLevel {
    Info,
    Ok,
    Warn
}

impl Logger {
    fn info(&mut self, msg: String) {
        self.lines.push(LogLine { level: LogLevel::Info, msg, highlight: None });
    }
    fn warn(&mut self, msg: String) {
        self.lines.push(LogLine { level: LogLevel::Warn, msg, highlight: None });
    }
    fn ok_with_highlight(&mut self, msg: String, highlight: String) {
        self.lines.push(LogLine { level: LogLevel::Ok, msg, highlight: Some(highlight) });
    }

    fn add_group(&mut self, title: String, affected_count: usize) {
        self.groups.push(GroupSummary { title, affected_count });
    }
}

fn render_ui(logger: &Logger, err: Option<String>) -> io::Result<()> {
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
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(size);

        let title = Paragraph::new(Line::from(vec![
            Span::styled("mac-setup", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" — Setup Summary"),
        ]))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = logger
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
            // Show group summaries with counts
            let mut summary_parts: Vec<String> = Vec::new();
            for g in &logger.groups {
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

    // Immediately restore terminal and return after drawing once
    disable_raw_mode()?;
    Ok(())
}
