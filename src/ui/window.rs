
use ratatui::{
    prelude::*,
    widgets::Paragraph,
    layout::{Layout, Constraint, Rect, 
        Direction::{Horizontal, Vertical}
    }, 
    Frame,
    widgets::Tabs,
};
use std::rc::Rc;
pub struct Window {
    text: String,
    layout: Rc<[Rect]>,
}

impl Window {

    pub fn new() -> Self {
        Window {
            text: "window".to_string(),
            layout: Rc::new([Rect::new(0,0,0,0)]),
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

        todo!(" top = Tabs::new()");

        // self.layout.push(Rc::new([Rect::new(2, 2, 2, 2)]))
    }

    pub fn for_kb(self) -> Rect {
        return self.layout[1];
    }

}
