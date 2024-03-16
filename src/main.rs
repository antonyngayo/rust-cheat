use clap::{Parser, Subcommand};
use serde_json;
use std::path::{Path, PathBuf};
use std::{
    fs,
    io::{self},
    vec,
};
use unicase::UniCase; // this helps with case insensitivity
mod configuration;
mod gitops;
mod tries;
mod utils;

use crate::configuration::Config;
// use tries::TrieStructure;
use utils::{choose_editor, create_config, perform_edit, perform_text_dump, read_files};

#[derive(Parser, Debug)]
#[clap(author = "Groctech", version, about = "Cheet sheet manager")]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

// thinking of introducing Clap for command line arguments
#[derive(Subcommand, Debug)]
enum Command {
    Init,
    List,
    Edit {
        #[clap(short = 'e', long = "edit")]
        flag: bool,

        file_name: String,
    },
    Read {
        #[clap(short = 'r', long = "read")]
        flag: bool,

        file_name: String,
    },
}

// clap struct
fn main() -> Result<(), io::Error> {
    // global variables
    let mut config = configuration::Config::new();
    let home_dir = dirs::home_dir().unwrap();
    let cheat_folder = Path::new(&home_dir).join(".cheat");
    let files = read_files(&cheat_folder)?; // getting the whole list of files in the directory OR an error
    let mut selector = vec![];
    let binary_base_path = PathBuf::from("/usr/bin/");
    let binaries = vec!["nano", "vi", "vim", "nvim", "emacs", "ee"];

    // use clap
    let args = Args::parse();

    match args.command {
        Command::Init => {
            match create_config(&config) {
                (true, path) => {
                    // loading configs into program
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let conf_text: Config = serde_json::from_str(&content).unwrap();
                            config = conf_text;
                        }
                        Err(_) => eprintln!("The config file is empty"),
                    }
                    config.config_path = path.to_string_lossy().to_string().to_owned(); // setting the config path to be written to disk soon
                    fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
                }
                (false, _) => eprintln!("An error occured while creating config file"),
            }
            match &config.editor_path {
                Some(editor) => {
                    if !editor.exists() {
                        choose_editor(binaries, &binary_base_path, &mut selector, &mut config)
                            .unwrap()
                    }
                } // checks whether the editor path is set in json file
                None => {
                    match choose_editor(binaries, &binary_base_path, &mut selector, &mut config) {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("An error occured while choosing an editor: {error}")
                        }
                    }
                }
            }

            // implementing the trie structure for quick file search

            // let mut file_name_list = TrieStructure::new();
            // for file_name in &files {
            //     file_name_list.insert(file_name.0.to_string());
            // }
        }
        Command::List => {
            for file in &files {
                // looping through HashMap contents
                println!(
                    "{:indent$} {}",
                    &file.1.name,
                    &file.1.path.to_string(),
                    indent = 40
                );
            }
        }
        Command::Edit {
            mut flag,
            file_name,
        } => {
            flag = true;
            assert!(flag == true);
            if !files.contains_key(&UniCase::new(file_name.clone())) {
                // performing a `LOOKUP` in the HashMap for fast search
                // check whether file exists; if not, we create a new one with the prescribed name
                let new_file = Path::new(&cheat_folder).join(&file_name); // preparing file name
                let result = fs::File::create(&new_file); // creating the file
                match result {
                    // checking whether the create function succeeded
                    Ok(_) => perform_edit(&config.editor_path.as_ref().unwrap(), new_file), // if operation succeeds,
                    Err(_) => eprintln!("An error occured while creating file"),
                }
            } else {
                let file_path =
                    PathBuf::from(files.get(&UniCase::new(file_name)).unwrap().path.clone());
                match config.editor_path.as_ref() {
                    Some(editor) => perform_edit(&editor, file_path),
                    None => {
                        match choose_editor(binaries, &binary_base_path, &mut selector, &mut config)
                        {
                            Ok(_) => perform_edit(&config.editor_path.as_ref().unwrap(), file_path),
                            Err(error) => {
                                eprintln!("An error occured while choosing an editor: {error}")
                            }
                        }
                    }
                }
            }
        }
        Command::Read { flag, file_name } => match files.get(&UniCase::new(file_name.clone())) {
            None => {
                eprintln!("The file does not exist");
            }
            Some(file_name) => match perform_text_dump(&PathBuf::from(&file_name.path)) {
                Ok(_) => {
                    assert!(flag == true)
                }
                Err(error) => println!("Unable to read file: {error}"),
            },
        },
    }
    Ok(())
}
