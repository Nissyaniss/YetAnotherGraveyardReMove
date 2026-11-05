use std::{path::PathBuf, process::exit};

use crate::{YagError, config::get_trash_dir};

pub struct State {
    pub config_file: PathBuf,
    pub trash_dir: PathBuf,
}

impl State {
    pub fn new() -> Result<Self, YagError> {
        let home_dir = dirs::config_dir().unwrap_or_else(|| {
            eprintln!("Error getting config directory.");
            exit(1)
        });
        let config_file = home_dir.join("Yagrm/config.toml");
        Ok(Self {
            config_file: config_file.clone(),
            trash_dir: get_trash_dir(&config_file)?,
        })
    }
}
