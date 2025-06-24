mod git_tui;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, prelude::Backend};

use git_tui::ui;

pub struct GitApp;

impl GitApp {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
