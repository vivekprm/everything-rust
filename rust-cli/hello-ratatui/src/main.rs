use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::io::{Result, stdout};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen);
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //main loop
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}
