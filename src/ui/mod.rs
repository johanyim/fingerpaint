pub mod keyboard;
pub mod window;
use keyboard::Keyboard;
use window::Window;

use crate::palette::Palette;

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
    layout::{Layout, Constraint, Rect, 
        Direction::{Horizontal, Vertical}
    }, 
    Frame,
};
use std::{rc::Rc, io::Stdout};
use std::io::{stdout, Result};

pub fn keyboard_selection(terminal: &mut Terminal, palette: &Palette) -> Result<Option<char>> {

    let selection: Result<Option<char>>;

    let keys = vec![
        "`1234567890-=", 
        "qwertyuiop[]", 
        "asdfghjkl;'", 
        "zxcvbnm,./"];

    let mut kb = Keyboard::new(keys, vec![0,8,10,14]);
    let mut window = Window::new();

    window.set_area(terminal.size()?, 1, 1);
    kb.set_area(window.for_kb());

    // kb.set_area(terminal.size()?);

    loop {
        //draw terminal
        terminal.draw(|frame| {
            // // this is for dynamically sizing keys
            // let kb_layout = layout_keys(frame.size(), keyboard.clone());
            kb.set_area(frame.size());
            
            kb.render(frame, palette);
        })?;


        //handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                // if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                if key.kind == KeyEventKind::Press {

                    if let KeyCode::Char(c) = key.code {
                        selection = Ok(Some(c)); break
                    }
                     
                    if let KeyCode::Esc = key.code {
                        selection = Ok(None); break
                    }
                }
            }
        }
    }

    return selection;
}

pub fn start_terminal() -> Result<Terminal> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    return Ok(terminal)
}


pub fn restore_terminal() -> Result<()>{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(())
}


