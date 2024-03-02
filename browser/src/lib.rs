use ratatui::{prelude::*, widgets::*};
use render::render;

mod parser;
mod render;

pub enum Mode {
    VISUAL,
    COMMAND,
    INSERT,
}

pub fn render_main_frame(frame: &mut Frame, html: String) {
    render::render(frame, render::PAGE_TYPE::INDEX, html);
}

pub fn render_frame(frame: &mut Frame, page_type: render::PAGE_TYPE, html: String) {
    render::render(frame, page_type, html)
}

fn clear() {
    print!("\x1B[2J");
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

    pub fn navigate_to_url(&mut self, url: String) {
        println!("{}", self.query(url));
    }

    pub fn navigate_home(&mut self) {
        println!("{}", self.query(self.index.clone()));
    }

    pub fn set_url(&mut self, url: String) {
        self.query = url
    }

    // TODO make this return a Result
    fn query(&mut self, url: String) -> String {
        let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
        return resp.to_string();
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn display(&mut self) {}
}
