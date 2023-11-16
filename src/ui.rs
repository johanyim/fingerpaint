//use crate::color::{Color, Format};
use crate::palette::Palette;

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


struct Keyboard<'a> {
    keys: Vec<&'a str>,
    offsets: Vec<u16>,
    layout: Vec<Rc<[Rect]>>,
    key_height: u16,
    key_width: u16,
}

//reduce writing
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

impl<'a> Keyboard<'_> {
    
    fn new<'b>(keys: Vec<&'b str>, offsets: Vec<u16>) -> Keyboard<'b> {
        return Keyboard{
            keys, 
            offsets,
            layout: vec![],
            key_height: 6,
            key_width: 12,
        }
    }

    fn set_area(&mut self, area: Rect) {

        //horizontal and vertical constraints
        let mut v_constraints: Vec<Constraint> = Vec::new();
        let mut h_constraints: Vec<Vec<Constraint>> = Vec::new();

        //populate them
        for (i, kb_row) in self.keys.iter().enumerate() {
            v_constraints.push(Constraint::Length(self.key_height));
            let mut row: Vec<Constraint> = Vec::new();
            row.push(Constraint::Length(self.offsets[i]));
            for _ in 0..kb_row.len() {
                // row.push(Constraint::Ratio(1,kb_row.len() as u32));
                row.push(Constraint::Length(self.key_width));
            }
            h_constraints.push(row);
        }

        //top and bottom padding
        let kb_height = self.key_height*(self.keys.len() as u16);
        let ud_padding = Layout::default()
            .direction(Vertical)
            .constraints([
                Constraint::Length((area.height - kb_height)/2),
                Constraint::Length(kb_height),
                Constraint::Length((area.height - kb_height)/2),
            ])
            .split(area);


        //vertical layouts
        let v_layout = Layout::default()
            .direction(Vertical)
            .margin(0)
            .constraints(v_constraints)
            .split(ud_padding[1]);

        //horizontal layouts = Vector of layouts
        let mut h_layouts: Vec<Rc<[Rect]>> = Vec::new();
        for i in 0..self.keys.len(){
            let h_layout = Layout::default()
                .direction(Horizontal)
                .margin(0)
                .constraints(h_constraints[i].clone())
                .split(v_layout[i]);
            h_layouts.push(h_layout);
        }

        self.layout = h_layouts;
    } 

    fn render(&self, frame: &mut Frame, palette: &Palette) {
        for i in 0..self.keys.len(){
            for (j,key) in self.keys[i].chars().enumerate(){

                let text = format!("{}\n{}", palette.get_name(key), key); 
                frame.render_widget(
                    Paragraph::new(text)
                    .style(palette.get_displayable(key)),
                    self.layout[i][j+1],
                    );
            }
        }
    }

    fn position(&self, row: usize, col: usize) -> Rect{
        return self.layout[row][col+1];
    }

}

pub fn keyboard_selection(terminal: &mut Terminal, palette: &Palette) -> Result<Option<char>> {

    let selection: Result<Option<char>>;

    let keys = vec![
        "`1234567890-=", 
        "qwertyuiop[]", 
        "asdfghjkl;'", 
        "zxcvbnm,./"];

    let mut kb = Keyboard::new(keys, vec![0,8,10,14]);
    kb.set_area(terminal.size()?);

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


