pub mod keyboard;
pub mod window;
pub mod tabline;
use keyboard::Keyboard;
use window::Window;
use tabline::Tabline;

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

pub fn color_select(terminal: &mut Terminal, 
              palettes: &mut Vec<Palette>, 
              palette_index: &mut usize) -> Result<Option<char>> {

    // let selection: Result<Option<char>>;

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
            window.set_area(frame.size(), 3, 5);
            kb.set_area(window.for_kb());
            
            window.render(frame);
            kb.render(frame, &palettes[*palette_index]);
        })?;


        //handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char(c) = key.code {
                        return Ok(Some(c));
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(None);
                    }
                    if let KeyCode::Right = key.code {
                        *palette_index += 1;
                    }
                    if let KeyCode::Left = key.code {
                        *palette_index -= 1; 
                    }
                }
            }
        }
    }
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


