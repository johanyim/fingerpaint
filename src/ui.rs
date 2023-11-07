//use crate::color::{Color, Format};
use crate::palette::Palette;


use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};


pub fn keyboard_selection(palette: &Palette) -> Result<char> {
    
    let keyboard = vec![
        "`1234567890-=", 
        "qwertyuiop[]", 
        "asdfghjkl;'", 
        "zxcvbnm,./"];


    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let selection: char;
    loop {
        //draw terminal
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
                area,
                );
        })?;


        //handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                // if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {selection = c; break},
                        _ => (),
                    }
                    // break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    return Ok(selection);

}

