use std::io;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self,
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let _args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        title: format!("asciiLife"),
        console_input: String::new(),
        selected_menu_item: 0,
        ..Default::default()
    };
    let app_result = app.run(&mut terminal);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    app_result
}

#[derive(Debug, Default)]
pub struct App<'a> {
    title: String,
    exit: bool,
    console_input: String,
    console_style: Style,
    menu_items: Vec<ListItem<'a>>,
    selected_menu_item: usize,
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        while !self.exit {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key_event) = event::read()? {
                    self.handle_key_event(key_event);
                }
            }
            self.draw(terminal)?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Enter => {
                    // Handle the entered command here
                    match self.console_input.as_str() {
                        "quit" => self.exit = true,
                        "exit" => self.exit = true,
                        "play" => {
                            self.console_style = Style::default().fg(ratatui::style::Color::Green);
                            self.console_input = "Playing...".to_string();
                        }
                        _ => {
                            self.console_style = Style::default().fg(ratatui::style::Color::Red);
                        }
                    }
                }
                KeyCode::Char(c) => {
                    self.console_input.push(c);
                }
                KeyCode::Backspace => {
                    self.console_input.pop();
                    self.console_style = ratatui::style::Style::default();
                }
                KeyCode::Up => {
                    if self.selected_menu_item > 0 {
                        self.selected_menu_item -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.selected_menu_item < self.menu_items.len() - 1 {
                        // Assuming 2 menu items for now
                        self.selected_menu_item += 1;
                    }
                }
                KeyCode::Right => {
                    // change the console input to the selected menu item
                    match self.selected_menu_item {
                        0 => self.console_input = "play".to_string(),
                        1 => self.console_input = "options".to_string(),
                        2 => self.exit = true,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        terminal.clear()?;

        let title = Paragraph::new(ratatui::style::Stylize::italic(
            ratatui::style::Stylize::bold(self.title.clone()),
        ))
        .block(Block::default().borders(Borders::ALL).title("Title"))
        .alignment(Alignment::Center);

        let mut state = ListState::default();
        state.select(Some(self.selected_menu_item));

        self.menu_items = vec![
            ListItem::new("Play").style(if self.selected_menu_item == 0 {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                ratatui::style::Style::default()
            }),
            ListItem::new("Options").style(if self.selected_menu_item == 1 {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                ratatui::style::Style::default()
            }),
            ListItem::new("Quit").style(if self.selected_menu_item == 2 {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                ratatui::style::Style::default()
            }),
        ];

        // center the menu horizontally
        let menu = List::new(self.menu_items.clone())
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .highlight_style(
                ratatui::style::Style::default()
                    .bg(ratatui::style::Color::DarkGray)
                    .fg(ratatui::style::Color::Black),
            );

        let console = Paragraph::new(self.console_input.clone())
            .block(Block::default().borders(Borders::ALL).title("Console"))
            .style(self.console_style)
            .alignment(Alignment::Left);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            );

        terminal.draw(|f| {
            let chunks = chunks.split(f.area());
            f.render_widget(title, chunks[0]);
            f.render_stateful_widget(menu, chunks[1], &mut state);
            f.render_widget(console, chunks[2]);
        })?;

        Ok(())
    }
}
