use ratatui::{prelude::*, widgets::*};

pub enum PAGE_TYPE {
    INDEX,
    LANDING,
    PAGE,
    ERROR,
}

/// Public facing function for rendering the TUI
///
/// # Arguments
///
/// * 'frame' - An instance of a Ratatui frame
/// * 'page_type' - An instance of the PAGE_TYPE enum defining the type of page to be rendered
///
/// # Returns
///
/// * No return
pub fn render(frame: &mut Frame, page_type: PAGE_TYPE, html: String) {
    match page_type {
        PAGE_TYPE::INDEX => index(frame),
        PAGE_TYPE::LANDING => landing(frame, html),
        PAGE_TYPE::PAGE => page(frame, html),
        PAGE_TYPE::ERROR => error(frame, html),
        _ => println!("Invalid Render Route"),
    }
}

/// Renders the index/main page
///
/// # Arguments
///
/// * 'frame' - An instance of a Ratatui frame
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

/// Renders the landing page
///
/// # Arguments
///
/// * 'frame' - An instance of a Ratatui frame
///
/// # Returns
///
/// * No Return
pub fn landing(frame: &mut Frame, html: String) {
    unimplemented!("This route hasn't been implemented")
}

/// Renders a web page
///
/// # Arguments
///
/// * 'frame' - An instance of a Ratatui frame
///
/// # Returns
///
/// * No Return
pub fn page(frame: &mut Frame, html: String) {
    unimplemented!("This route hasn't been implemented")
}

/// Renders an error page
///
/// # Arguments
///
/// * 'frame' - An instance of a Ratatui frame
///
/// # Returns
///
/// * No Return
pub fn error(frame: &mut Frame, html: String) {
    unimplemented!("This route hasn't been implemented")
}
