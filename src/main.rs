mod App;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::cursor;
use ratatui::layout::Rect;
use ratatui::{crossterm, Terminal};
use std::io::{self};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut app = App::App::new();
    let mut cursor: u8 = 0;

    loop {
        let size = terminal.size()?;
        if size.height < 30 || size.width < 80 {
            terminal.draw(|f| {
                f.render_widget(
                    ratatui::widgets::Paragraph::new(
                        "Please resize the terminal to at least 80x30.",
                    )
                    .block(
                        ratatui::widgets::Block::default()
                            .borders(ratatui::widgets::Borders::ALL)
                            .title("Error"),
                    )
                    .alignment(ratatui::layout::Alignment::Center),
                    Rect::new(0, 0, size.width, size.height),
                )
            })?;

            if event::poll(std::time::Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }

            continue;
        }

        terminal.draw(|f| app.draw(f, "asciiLife", &input, &mut cursor))?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                        app.console_style = ratatui::style::Style::default();
                    }
                    KeyCode::Enter => match app.handle_input(&input) {
                        Ok(true) => input.clear(),
                        Ok(false) => break,
                        Err(_) => (),
                    },
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
