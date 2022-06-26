use std::{io::{self}, path::{PathBuf, Path},vec, process::exit, fs};
use clap::{Parser, Subcommand};
use serde_json;
use unicase::UniCase; // this helps with case insensitivity
mod configuration;
mod utils;

use utils::{create_config, perform_edit, read_files, choose_editor};

use crate::configuration::Config;

/// simple program to record cheatsheets
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name="cheat")]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand )]
enum Commands {
    /// Subcommand for listing everything in the .cheat folder
    L {},
    /// Subcommand to edit a file in the folder
    E {
        name: String
    },
    /// Subcommand to delete a file in the folder
    D {
        name: String
    },
    /// Subcommand to search for a phrase in all the files
    S {
        term: String
    },
}
fn main() -> Result<(), io::Error>{
    // global variables
    let mut config = configuration::Config::new();
    let home_dir = dirs::home_dir().unwrap();
    let cheat_folder = Path::new(&home_dir).join(".cheat");
    let mut files = read_files(&cheat_folder)?; // getting the whole list of files in the directory OR an error
    let mut selector = vec![];
    let binary_base_path = PathBuf::from("/usr/bin/");
    let binaries = vec!["nano", "vi","vim", "nvim", "emacs", "ee"];
    // command line parsing 
    let args = Cli::parse();

    match create_config(&config){
        (true, path) => {
            // loading configs into program
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let conf_text: Config = serde_json::from_str(&content).unwrap();
                    config = conf_text;
                },
                Err(_) => eprintln!("The config file is empty")
            }
            config.config_path = path.to_string_lossy().to_string().to_owned(); // setting the config path to be written to disk soon
            fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
        },
        (false, _) => eprintln!("An error occured while creating config file")
    }
    match &config.editor_path {
        Some(editor) => { if !editor.exists(){ choose_editor(binaries, &binary_base_path, &mut selector, &mut config) } }, // checks whether the editor path is set in json file
        None => { choose_editor(binaries, &binary_base_path, &mut selector, &mut config) } // assumes editor not set in json and forces a set by user
    }
    match args.command {
        Commands::L {  } => { 
            // let files_content = files.drain();
            // files.sort();
            for file in files.drain() {
                println!("{:indent$} {}", &file.1.name, &file.1.path.to_string(), indent=40);
            }
        },
        Commands::D { name } => {
            eprintln!("{}", name)
        },
        Commands::E { name } => {
            if !files.contains_key(&UniCase::new(name.clone())) { // performing a `LOOKUP` in the HashMap for fast search
                // check whether file exists; if not, we create a new one with the prescribed name
                let new_file = Path::new(&cheat_folder).join(&name); // preparing file name
                let result = fs::File::create(&new_file); // creating the file
                match result { // checking whether the create function succeeded
                    Ok(_) => perform_edit(&config.editor_path.unwrap(), new_file), // if operation succeeds, we open for editing
                    Err(error) => eprintln!("An error occurred when creating file: {}", error)
                }
                exit(1);
            }
            let p = &files.get(&UniCase::new(name)).unwrap().path; // checking whether the file exists in a CASE INSESNITIVE manner
            perform_edit(&config.editor_path.unwrap(), PathBuf::from(p)); // opening file for edit
        },
        Commands::S { term } => { eprintln!("{}", term) }
    }
    Ok(())
}