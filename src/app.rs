use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

enum Scene {
    Main,
    Play,
    Options,
}

impl Scene {
    fn fmt(&self) -> String {
        match self {
            Scene::Main => "Main".to_string(),
            Scene::Play => "Play".to_string(),
            Scene::Options => "Options".to_string(),
        }
    }
}

pub(crate) struct App {
    scene: Scene,
    pub(crate) console_style: Style,
}

impl App {
    pub fn new() -> App {
        App {
            scene: Scene::Main,
            console_style: Style::default(),
        }
    }

    pub fn draw(&self, f: &mut Frame, title: &str, input: &str, _cursor: &mut u8) {
        let title_wg = Paragraph::new(ratatui::style::Stylize::bold(title))
            .block(Block::default().borders(Borders::ALL).title("Title"))
            .alignment(Alignment::Center);

        let screen_wg = Paragraph::new(self.scene.fmt())
            .block(Block::default().borders(Borders::ALL).title("Scene"))
            .alignment(Alignment::Center);

        let show_cursor: String;
        if *_cursor % 2 == 0 {
            show_cursor = "|".to_string();
        } else {
            show_cursor = " ".to_string();
        }

        *_cursor += 1;

        let console_wg =
            Paragraph::new(format!("🔥 {}{}", input, show_cursor)) // Adding a cursor symbol '|'
                .block(Block::default().borders(Borders::ALL).title("Console"))
                .style(self.console_style)
                .alignment(Alignment::Left);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            // .margin(2)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.area());

        f.render_widget(title_wg, chunks[0]);
        f.render_widget(screen_wg, chunks[1]);
        f.render_widget(console_wg, chunks[2]);
    }

    pub fn handle_input(&mut self, input: &str) -> Result<bool, bool> {
        match input {
            "quit" | "exit" => {
                return Ok(false);
            }
            "play" => {
                self.console_style = Style::default();
                self.scene = Scene::Play;
            }
            "options" | "settings" => {
                self.console_style = Style::default();
                self.scene = Scene::Options;
            }
            "main" | "home" => {
                self.console_style = Style::default();
                self.scene = Scene::Main;
            }
            _ => {
                self.console_style = Style::default().fg(Color::Red);
                return Err(true);
            }
        }

        Ok(true)
    }
}
