use ratatui::{prelude::*, widgets::*};
use render::render;

mod parser;
mod render;

pub enum Mode {
    VISUAL,
    COMMAND,
    INSERT,
}

pub fn render_frame(frame: &mut Frame) {
    render::render(frame, render::PAGE_TYPE::INDEX);
}

pub struct Browser {
    index: String,
    query: String,
    tabs: i64,
    mode: Mode,
}

impl Browser {
    pub fn new() -> Self {
        Browser {
            index: "https://www.google.com".to_string(),
            query: "".to_string(),
            tabs: 1,
            mode: Mode::VISUAL,
        }
    }

    pub fn navigate_home(&mut self) {
        println!("{}", self.query(self.index.clone()));
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn query(&mut self, url: String) -> String {
        let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
        return resp;
    }

    fn display(&mut self) {}
}
