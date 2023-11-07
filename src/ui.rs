//use crate::color::{Color, Format};
use crate::palette::Palette;


use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Paragraph},
    style::{Style,Color},
    layout::{Layout, Constraint}, Frame,
};
use std::io::{stdout, Result};


pub fn keyboard_selection(palette: &Palette) -> Result<char> {
    
    let keyboard = vec![
        "`1234567890-=", 
        "qwertyuiop[]", 
        "asdfghjkl;'", 
        "zxcvbnm,./"];



    // let inner_layout = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(vec![
    //                  Constraint::Percentage(25),
    //                  Constraint::Percentage(75),
    //     ])
    //     .split(outer_layout[1]);
    

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let selection: char;

    let c: char = 'q';
    loop {
        //draw terminal
        terminal.draw(|frame| {

            let v_layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(vec![
                             Constraint::Length(3),
                             Constraint::Length(3),
                             Constraint::Length(3),
                             Constraint::Length(3),
                ])
                .split(frame.size());

            let h_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints(vec![
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                             Constraint::Ratio(1, 12),
                ])
                .split(v_layout[0]);

            let area = frame.size();
            let text = palette.get_name(c);
            let col = palette.get_rgba(c);

            frame.render_widget(
                Paragraph::new(palette.get_name('q'))
                .style(palette.get_displayable('q')),
                h_layout[0],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('w'))
                .style(palette.get_displayable('w')),
                h_layout[1],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('e'))
                .style(palette.get_displayable('e')),
                h_layout[2],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('r'))
                .style(palette.get_displayable('r')),
                h_layout[3],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('t'))
                .style(palette.get_displayable('t')),
                h_layout[4],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('y'))
                .style(palette.get_displayable('y')),
                h_layout[5],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('u'))
                .style(palette.get_displayable('u')),
                h_layout[6],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('i'))
                .style(palette.get_displayable('i')),
                h_layout[7],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('o'))
                .style(palette.get_displayable('o')),
                h_layout[8],
                );
            frame.render_widget(
                Paragraph::new(palette.get_name('p'))
                .style(palette.get_displayable('p')),
                h_layout[9],
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




