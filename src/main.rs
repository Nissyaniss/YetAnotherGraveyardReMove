use clap::{CommandFactory, Parser};
use clap_complete::{Generator, generate};
use dialoguer::Confirm;
use skim::prelude::*;
use state::State;
use std::env;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::{self};
use std::path::Path;
use std::process::exit;
use std::{fs, path::PathBuf};

mod args;
mod config;
mod error;
mod state;

use args::Args;

use crate::error::{BrmError, BrmErrorUtils, BrmErrorUtilsMessage};

fn main() -> Result<(), BrmError> {
    let args = Args::parse();
    let mut files = args.paths.clone();
    let utils = State::new()?;

    if std::env::args().len() == 1 {
        Args::command()
            .print_help()
            .to_brm_error_message("Failed to print help")?;
        exit(0);
    }

    if args.delete_trash_contents {
        delete_trash_contents(&utils)?;
    }

    if args.trash_path_reveal {
        println!(
            "This is the current trash directory :\n{}",
            utils.trash_dir.display()
        );
        exit(0)
    }

    if let Some(shell) = args.generate_completions {
        generate_completions(shell);
        exit(0);
    }

    if let Some(new_trash_path_path) = args.new_trash_path.clone() {
        new_trash_path(new_trash_path_path.as_path(), &utils)?;
    }

    if args.fzf {
        files = fzf(files)?;
    }

    trashing(files, &utils, &args)?;
    Ok(())
}

fn generate_completions<G: Generator>(r#gen: G) {
    let mut cmd = Args::command();
    let Ok(bin_name) = env::current_exe() else {
        eprintln!("Cannot get binary name");
        exit(1);
    };
    let Some(file_name) = bin_name.file_name() else {
        eprintln!("Binary path is finishing with `.` or `..`. Why ?");
        exit(1)
    };
    let bin_name = file_name.to_string_lossy().to_string();
    generate(r#gen, &mut cmd, &bin_name, &mut io::stdout());
}

fn delete_trash_contents(utils: &State) -> Result<(), BrmError> {
    let confirmed = Confirm::new()
        .with_prompt("Are you sure you want to delete all your trash ?")
        .default(false)
        .interact()
        .to_brm_error()?;
    if confirmed {
        for entry in read_dir(utils.trash_dir.clone()).to_brm_error_message(format!(
            "Failed to read directory {}",
            utils.trash_dir.display()
        ))? {
            let path = entry
                .to_brm_error_message("Failed to read directory entry")?
                .path();
            if path.is_file() {
                remove_file(&path)
                    .to_brm_error_message(format!("Error removing file {}", path.display()))?;
            } else {
                remove_dir_all(&path)
                    .to_brm_error_message(format!("Error removing directory {}", path.display()))?;
            }
        }
    }
    Ok(())
}

fn new_trash_path(new_trash_path: &Path, utils: &State) -> Result<(), BrmError> {
    if new_trash_path.is_file() {
        println!("The new trash path must be a directory !");
        exit(1);
    }
    let new_trash_path_display = new_trash_path.display();
    fs::write(
        utils.config_file.clone(),
        format!("path_to_trash = '{new_trash_path_display}'"),
    )
    .to_brm_error_message("Error while writing to config")?;
    println!("New path is now : {new_trash_path_display}");
    exit(0);
}

fn fzf(mut files: Vec<PathBuf>) -> Result<Vec<PathBuf>, BrmError> {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .build()
        .to_brm_error()?;

    let selected_items = Skim::run_with(options, None).to_brm_error()?.selected_items;

    for item in &selected_items {
        files.push(PathBuf::from(item.text().into_owned()));
    }

    Ok(files)
}

fn trashing(files: Vec<PathBuf>, utils: &State, args: &Args) -> Result<(), BrmError> {
    for file in files {
        if file.exists() {
            if file.is_dir() {
                let confirmed = Confirm::new()
                    .with_prompt(
                        "The file you are trying to trash or remove is a directory. Are you sure ?",
                    )
                    .default(false)
                    .interact()
                    .to_brm_error()?;

                if confirmed {
                    move_remove_file(args.force, &file, utils)?;
                }
            } else {
                move_remove_file(args.force, &file, utils)?;
            }
        } else {
            println!("File or directory does not exist : {}", file.display());
        }
    }

    Ok(())
}

fn move_remove_file(force: bool, file: &PathBuf, utils: &State) -> Result<(), BrmError> {
    if force {
        fs::remove_file(file).to_brm_error_message("Failed to remove file")?;
    }

    let Some(file_name) = file.file_name() else {
        eprintln!("File is `.` or `..`.");
        exit(1);
    };
    let file_name_owned = file_name.to_os_string();
    let path_to_trash = utils.trash_dir.join(file_name);
    let is_numbered = path_to_trash.exists();
    let mut i = 1;
    if is_numbered {
        loop {
            let mut file_name_numbered = file_name_owned.clone();
            file_name_numbered.push(i.to_string());
            let path_to_trash = utils.trash_dir.join(file_name_numbered);
            if !path_to_trash.exists() {
                fs::rename(file, path_to_trash.clone()).to_brm_error_message(format!(
                    "Failed to move {} to {}",
                    file_name.display(),
                    path_to_trash.display()
                ))?;
                break;
            }
            i += 1;
        }
    } else {
        fs::rename(file, path_to_trash.clone()).to_brm_error_message(format!(
            "Failed to move {} to {}",
            file_name.display(),
            path_to_trash.display()
        ))?;
    }

    Ok(())
}
