
use ratatui::{
    prelude::*,
    widgets::Paragraph,
    layout::{Layout, Constraint, Rect, 
        Direction::{Horizontal, Vertical}
    }, 
    Frame,
    widgets::{Tabs, Block, Wrap},
};
use crate::palette::Palette;
use std::{rc::Rc, path::Path};
pub struct Window {
    text: String,
    layout: Rc<[Rect]>,
    palettes: Vec<Palette>
}

use std::fs;
impl Window {

    pub fn get_tabs(&mut self, path: &Path) {
        
        
        let pathlist = fs::read_dir(path).unwrap();
        for path in pathlist {
            println!("{}", path.unwrap().path().display());
        }

    }

    pub fn new() -> Self {
        Window {
            text: "window".to_string(),
            layout: Rc::new([Rect::new(0,0,0,0)]),
            palettes: vec![],
        }
    }

    pub fn set_area(&mut self,  area: Rect, top: u16, bot: u16) {
        let v_contraints = Layout::default()
            .direction(Vertical)
            .constraints([
                Constraint::Length(top),
                Constraint::Length(area.height- top - bot),
                Constraint::Length(bot),
            ])
            .split(area);

        self.layout = v_contraints;

        // self.layout.push(Rc::new([Rect::new(2, 2, 2, 2)]))
    }

    pub fn for_kb(&self) -> Rect {
        return self.layout[1];
    }
    
    pub fn render(&self, frame: &mut Frame) {
        todo!("make functioning tab line");
        let titles = vec!["palette1", "palette2", "palette3"];
        frame.render_widget( Tabs::new(titles), self.layout[0]);
        frame.render_widget( Paragraph::new("Press [esc] to exit")
                             .alignment(Alignment::Center), self.layout[2])
    }


    

}
