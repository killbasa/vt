use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};
use std::time::Duration;

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
