use ratatui::{prelude::*, widgets::*};

pub enum PAGE_TYPE {
    INDEX,
    LANDING,
    PAGE,
    ERROR,
}

pub fn render(frame: &mut Frame, page_type: PAGE_TYPE) {
    match page_type {
        PAGE_TYPE::INDEX => index(frame),
        PAGE_TYPE::LANDING => landing(frame),
        PAGE_TYPE::PAGE => page(frame),
        PAGE_TYPE::ERROR => error(frame),
        _ => println!("Invalid Render Route"),
    }
}

/// Renders the index/main page
///
/// # Arguments
///
/// * 'frame' - An instance of a Rratatui frame
///
/// # Returns
///
/// * No return
pub fn index(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("NAB"),
        main_layout[0],
    );
    frame.render_widget(Block::new().title(":"), main_layout[2]);

    frame.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Web Window").borders(Borders::ALL)),
        main_layout[1],
    );
    frame.render_widget(Paragraph::new(":"), frame.size())
}

pub fn landing(frame: &mut Frame) {
    unimplemented!("This route hasn't been implemented")
}

pub fn page(frame: &mut Frame) {
    unimplemented!("This route hasn't been implemented")
}

pub fn error(frame: &mut Frame) {
    unimplemented!("This route hasn't been implemented")
}
