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


struct Keyboard<'a> {
    keys: Vec<&'a str>,
    r_offsets: Vec<&'a str>,
}


pub fn keyboard_selection(palette: &Palette) -> Result<char> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let selection: char;

    let keyboard = vec![
        "`1234567890-=", 
        "qwertyuiop[]", 
        "asdfghjkl;'", 
        "zxcvbnm,./"];
    let h_layouts = layout_keys(terminal.size()?, keyboard.clone());
    loop {
        //draw terminal
        terminal.draw(|frame| {


            let h_layouts = layout_keys(frame.size(), keyboard.clone());
            render_keys(keyboard.clone(), h_layouts.clone(), frame, palette);





            // frame.render_widget(
            //     Paragraph::new(palette.get_name(c))
            //     .style(palette.get_displayable(c)),
            //     h_layout[0],
            //     );
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


use ratatui::layout::{Rect, Direction::{Horizontal, Vertical}};
use std::rc::Rc;
fn layout_keys(area: Rect, keyboard: Vec<&str>) -> Vec<Rc<[Rect]>> {
    //horizontal constraints
    let mut h_constraints: Vec<Vec<Constraint>> = Vec::new();
    //vertical constraints
    let mut v_constraints: Vec<Constraint> = Vec::new();
    for kb_row in keyboard.iter() {
        v_constraints.push(Constraint::Length(5));
        let mut row: Vec<Constraint> = Vec::new();
        for _ in 0..kb_row.len(){
            row.push(Constraint::Ratio(1,kb_row.len() as u32));
        }
        h_constraints.push(row);
    }

    //vertical layouts
    let v_layout = Layout::default()
        .direction(Vertical)
        .margin(1)
        .constraints(v_constraints)
        .split(area);

    //horizontal layouts = Vector of layouts
    let mut h_layouts: Vec<Rc<[Rect]>> = Vec::new();
    for i in 0..keyboard.len(){
        let h_layout = Layout::default()
            .direction(Horizontal)
            .margin(0)
            .constraints(h_constraints[i].clone())
            .split(v_layout[i]);
        h_layouts.push(h_layout);
    }

    return h_layouts;
}

fn render_keys(keyboard: Vec<&str>, layout:Vec<Rc<[Rect]>>, frame: &mut Frame, palette: &Palette) {



    for i in 0..keyboard.len(){
        for (j,key) in keyboard[i].chars().enumerate(){

            let text = format!("{}\n{}", palette.get_name(key), key); 
            frame.render_widget(
                Paragraph::new(text)
                .style(palette.get_displayable(key)),
                layout[i][j],
                );
        }

    }

}
