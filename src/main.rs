use std::{io::{self, stdin}, path::{PathBuf, Path},vec, process::exit, fs};
use clap::{Parser, Subcommand};
use serde_json;
use unicase::UniCase; // this helps with case insensitivity
mod configuration;
mod utils;

use utils::{create_config, check_for_editor, perform_edit, read_files};

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
    let mut config = configuration::Config::new();
    match create_config(&config){
        (true, path) => {
            // loading configs into program
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let conf_text: Config = serde_json::from_str(&content).unwrap();
                    println!("{:?}", conf_text.config_path);
                },
                Err(_) => eprintln!("The config file is empty")
            }
            config.config_path = path.to_string_lossy().to_string().to_owned(); // setting the config path to be written to disk soon
            fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
        },
        (false, _) => eprintln!("An error occured while creating config file")
    }

    let binary_base_path = PathBuf::from("/usr/bin/");
    let binaries = vec!["nano", "vi","vim", "nvim", "emacs", "ee"];
    let args = Cli::parse();


    // global cheat path
    let home_dir = dirs::home_dir().unwrap();
    let cheat_folder = Path::new(&home_dir).join(".cheat");


    let mut files = read_files(cheat_folder)?; // getting the whole list of files in the directory OR an error

    let mut selector = vec![];

    // I need to think whether `Option` was the best choice for this part
    match &config.editor_path {
        Some(path) => {eprintln!("We have a path {:?}", path)},
        None => { 
            let mut editor_selection = String::new();
            check_for_editor(binaries, &binary_base_path, &mut selector);
            println!("Please select an editor");
            for (index, editor) in selector.iter().enumerate() {
                println!("{}) {:?} ", index, editor);
            }
            let mut editor_index = 0;
            stdin().read_line(&mut editor_selection).ok().expect("An error occurred while capturing input");
            match editor_selection.trim().parse::<usize>() {
                Ok(num) => { editor_index = num },
                Err(e) => eprintln!("An error occurred during casting: {}", e)
            }
            config.editor_path = Some(selector[editor_index].clone()); // setting the editor
            fs::write(&config.config_path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
        }
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
            if !files.contains_key(&UniCase::new(name.clone())) {
                eprintln!("The file `{}` does not exist", &name);
                exit(1);
            }
            let p = &files.get(&UniCase::new(name)).unwrap().path;
            perform_edit(&config.editor_path.unwrap(), PathBuf::from(p));
        },
        Commands::S { term } => {
            eprintln!("{}", term)
        }

    }
    Ok(())
}