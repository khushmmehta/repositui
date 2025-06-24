mod git;
mod jujutsu;

use color_eyre::{Result, eyre::eyre};
use ratatui::{Terminal, prelude::Backend};
use std::fs;

use git::GitApp;
use jujutsu::JujutsuApp;

pub enum RepositoryType {
    Git,
    Jujutsu,
}

pub struct App {
    repo_type: RepositoryType,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut repo_type: Option<RepositoryType> = None;

        // NOTE: I am yet to decide which is the better approach
        // let paths = fs::read_dir(".")?
        //     .filter_map(Result::ok)
        //     .map(|entry| entry.path())
        //     .filter(|path| path.is_dir());

        for entry in fs::read_dir(".")? {
            let path = entry?.path();

            if !path.is_dir() {
                continue;
            }

            if path.ends_with(".jj") {
                repo_type = Some(RepositoryType::Jujutsu);
                break;
            } else if path.ends_with(".git") {
                repo_type = Some(RepositoryType::Git);
            }
        }

        match repo_type {
            Some(repo_type) => Ok(Self { repo_type }),
            None => Err(eyre!("no repository present")),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        match self.repo_type {
            RepositoryType::Git => GitApp::new()?.run(terminal),
            RepositoryType::Jujutsu => JujutsuApp::new()?.run(terminal),
        }
    }
}
