use std::io;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self,
        event::{self, KeyCode, KeyEventKind},
        style::Stylize,
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

#[derive(Debug)]
enum Scene {
    Main,
    Play,
    Options,
}

impl Default for Scene {
    fn default() -> Self {
        Scene::Main
    }
}

impl Scene {
    fn fmt(&self) -> &str {
        match self {
            Scene::Main => "Main",
            Scene::Play => "Play",
            Scene::Options => "Options",
        }
    }
}

#[derive(Debug)]
struct Console {
    input: String,
    style: Style,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            input: String::new(),
            style: Style::default(),
        }
    }
}

impl Console {
    fn reset(&mut self) {
        self.input.clear();
        self.style = Style::default();
    }
}

#[derive(Debug, Default)]
pub struct App {
    title: String,
    scene: Scene,
    exit: bool,
    console: Console,
}

impl App {
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
                    match self.console.input.as_str() {
                        "quit" | "exit" => {
                            self.exit = true;
                        }
                        "play" => {
                            self.console.reset();
                            self.scene = Scene::Play;
                        }
                        "options" | "settings" => {
                            self.console.reset();
                            self.scene = Scene::Options;
                        }
                        "main" | "home" => {
                            self.console.reset();
                            self.scene = Scene::Main;
                        }
                        _ => {
                            self.console.style = Style::default().fg(ratatui::style::Color::Red);
                        }
                    }
                }
                KeyCode::Char(c) => {
                    self.console.input.push(c);
                }
                KeyCode::Backspace => {
                    self.console.input.pop();
                    self.console.style = ratatui::style::Style::default();
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

        let screen = Paragraph::new(self.scene.fmt())
            .block(Block::default().borders(Borders::ALL).title("Scene"))
            .alignment(Alignment::Center);

        let console = Paragraph::new(self.console.input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Console"))
            .style(self.console.style)
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
            f.render_widget(screen, chunks[1]);
            f.render_widget(console, chunks[2]);
        })?;

        Ok(())
    }
}
