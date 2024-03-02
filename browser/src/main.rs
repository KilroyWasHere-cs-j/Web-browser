use browser;
use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    let mut command: Vec<String> = Vec::new();
    let mut brow = browser::Browser::new();
    // Create a new terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    //TODO move this into render
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        // TODO move to render
        terminal.draw(ui)?;
        let quit = handle_events(&mut brow, &mut command)?; // Extract the boolean value from the tuple
        should_quit = quit; // Assign the boolean value to should_quit
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn process_command(b: &mut browser::Browser, cmds: &mut Vec<String>) {
    let cmd = cmds.join("");
    cmds.clear();

    if cmd.contains("h") {
        b.navigate_home();
    } else if cmd.contains("s") {
        b.set_url("Hello World!1".to_string());
        b.navigate_to_url("https://www.google.com/".to_string());
    } else {
        print!("There was an error with command: {}", cmd);
    }
}

fn handle_events(b: &mut browser::Browser, cmd: &mut Vec<String>) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        cmd.push(c.to_string());
                        print!("{}", c)
                    }
                    KeyCode::Enter => {
                        process_command(b, cmd);
                    }
                    _ => {}
                }
                match key.modifiers {
                    KeyModifiers::CONTROL => match key.code {
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        KeyCode::Char('v') => {
                            let _ = &b.set_mode(browser::Mode::VISUAL);
                            return Ok(false);
                        }
                        KeyCode::Char('i') => {
                            let _ = &b.set_mode(browser::Mode::INSERT);
                            return Ok(false);
                        }
                        KeyCode::Char('c') => {
                            let _ = &b.set_mode(browser::Mode::COMMAND);
                            return Ok(false);
                        }
                        // No matching key was pressed for ctrl
                        _ => {}
                    },
                    _ => {
                        // No key modifiers pressed
                        return Ok(false);
                    }
                }
                match key.code {
                    KeyCode::Char(c) => {
                        return Ok(false);
                    }
                    _ => return Ok(false),
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let html = "".to_string();
    browser::render_main_frame(frame, html)
}
