use std::{
    fs::{File, create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
    process::exit,
};

use serde::{Deserialize, Serialize};

use crate::{YagError, error::YagErrorUtilsMessage};

#[derive(Deserialize, Serialize)]
pub struct Config {
    path_to_trash: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_dir().unwrap_or_else(|| {
            eprintln!("Cannot get $XDG_DATA_HOME");
            exit(1)
        });
        Self {
            path_to_trash: data_dir.join("Yagrm/trash/"),
        }
    }
}

fn create_if_dont_exist(config_file: &Path) -> Result<(), YagError> {
    if !config_file.exists() {
        let Some(config_file_parent) = config_file.parent() else {
            eprintln!("Config file has no parent.");
            exit(1);
        };
        create_dir_all(config_file_parent).to_yag_error_message("Failed to create directories")?;
        File::create(config_file).to_yag_error_message("Failed to create config file")?;
        write(
            config_file,
            toml::to_string(&Config::default())
                .to_yag_error_message("Failed to serialize config to write it")?,
        )
        .to_yag_error_message("Failed to write the config file")?;
    }
    Ok(())
}

pub fn get_trash_dir(config_file: &Path) -> Result<PathBuf, YagError> {
    create_if_dont_exist(config_file)?;
    let config_str =
        read_to_string(config_file).to_yag_error_message("Failed to read config file")?;
    let parsed_config: Config = toml::from_str(&config_str).to_yag_error_message("Config")?;
    if !PathBuf::from(&parsed_config.path_to_trash).exists() {
        create_dir_all(&parsed_config.path_to_trash)
            .to_yag_error_message("Error creating trash directory")?;
    } else if PathBuf::from(&parsed_config.path_to_trash).is_file() {
        println!("The trash path must be a directory.");
        exit(1);
    }
    Ok(parsed_config.path_to_trash)
}
