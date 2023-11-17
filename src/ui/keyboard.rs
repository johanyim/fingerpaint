use crate::palette::Palette;

use ratatui::{
    widgets::Paragraph,
    layout::{Layout, Constraint, Rect, 
        Direction::{Horizontal, Vertical}
    }, 
    Frame,
};
use std::rc::Rc;

pub struct Keyboard<'a> {
    keys: Vec<&'a str>,
    offsets: Vec<u16>,
    layout: Vec<Rc<[Rect]>>,
    key_height: u16,
    key_width: u16,
}

//reduce writing

impl<'a> Keyboard<'_> {
    
    pub fn new<'b>(keys: Vec<&'b str>, offsets: Vec<u16>) -> Keyboard<'b> {
        return Keyboard{
            keys, 
            offsets,
            layout: vec![],
            key_height: 6,
            key_width: 12,
        }
    }

    pub fn set_area(&mut self, area: Rect) {

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

        //remove overfill at bottom of keyboard
        let kb_height = self.key_height*(self.keys.len() as u16);
        let bound = Layout::default()
            .direction(Vertical)
            .constraints([
                Constraint::Length(kb_height),
                Constraint::Length(0),
            ])
            .split(area);


        //vertical layouts
        let v_layout = Layout::default()
            .direction(Vertical)
            .margin(0)
            .constraints(v_constraints)
            .split(bound[0]);

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
    
    fn position(&self, row: usize, col: usize) -> Rect{
        // first column used by offsets
        return self.layout[row][col+1];
    }

    pub fn render(&self, frame: &mut Frame, palette: &Palette) {

        for i in 0..self.keys.len(){
            for (j,key) in self.keys[i].chars().enumerate(){
                let text = format!("{}\n{}", palette.get_name(key), key); 
                frame.render_widget(
                    Paragraph::new(text)
                    .style(palette.get_displayable(key)),
                    self.position(i,j),
                );
            }
        }
    }


}

