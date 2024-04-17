use std::io::stdout;

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = startup()?;

    terminal.draw(|frame| {
        let area = frame.size();
        let status_bar = ratatui_statusbar::StatusBar::default()
            .section(0, "hello".into())
            .section(1, "world".into());
        frame.render_widget(status_bar, area);
    })?;

    while !matches!(event::read()?, Event::Key(_)) {}

    shutdown()?;

    Ok(())
}

fn startup() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, color_eyre::eyre::Error> {
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    enable_raw_mode()?;
    Ok(terminal)
}

fn shutdown() -> Result<(), color_eyre::eyre::Error> {
    disable_raw_mode()?;
    stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
