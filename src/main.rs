use std::io;
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    let mut term = create_term()?;
    term.draw(|mut f| {
        let size = f.size();
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, size);
    });
    Ok(())
}

fn create_term() -> Result<Terminal<impl tui::backend::Backend>, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);

    Terminal::new(backend)
}
