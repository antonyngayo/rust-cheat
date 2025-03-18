use std::{
    env, fs,
    io::{self},
    path::{Path, PathBuf},
    process::exit,
    vec,
};
use trie::tt::Trie;
use unicase::UniCase; // this helps with case insensitivity
mod configuration;
mod gitops;
mod trie;
mod utils;

// use tries::TrieStructure;

use utils::{choose_editor, create_config, perform_edit, perform_text_dump, read_files};

use crate::configuration::Config;

fn main() -> Result<(), io::Error> {
    // global variables
    let mut config = configuration::Config::new();
    let home_dir = dirs::home_dir().unwrap();
    let cheat_folder = Path::new(&home_dir).join(".cheat");
    let files = read_files(&cheat_folder)?; // getting the whole list of files in the directory OR an error
    let mut selector = vec![];
    let binary_base_path = PathBuf::from("/usr/bin/");
    let binaries = vec!["nano", "vi", "vim", "nvim", "emacs", "ee"];
    // getting commandline arguments
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.is_empty() {
        println!("too few arguments");
    }
    // creating a config file if it did not exist
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
            }
        } // checks whether the editor path is set in json file
        None => choose_editor(binaries, &binary_base_path, &mut selector, &mut config), // assumes editor not set in json and forces a set by user
    }

    // implementing the trie structure for quick file search

    // let mut file_name_list = TrieStructure::new();
    // for file_name in &files {
    //     file_name_list.insert(file_name.0.to_string());
    // }

    // parsing command line arguments
    if cli_args.len() < 2 {
        println!("There are no arguments passed"); // put a menu
        println!();
        println!("      == USAGE ==         ");
        println!();
        println!("  -l  -   List all files  ");
        println!();
        println!("  -e  -   Edit a file     ");
        println!();
        println!("  -s  -   Search a file   ");
        println!();
        println!("  -d  -   Delete a file   ");
        println!();
        println!("  -p  [Commit Message ]-   Push to git ");
        println!();
        exit(1);
    }
    if cli_args.len() == 2 && cli_args[1] == "-l" {
        // for listing all the file names in the .cheat folder
        for file in &files {
            // looping through HashMap contents
            println!(
                "{:indent$} {}",
                &file.1.name,
                &file.1.path.to_string(),
                indent = 40
            );
        }
    } else if cli_args[1] == "-s" && cli_args.len() == 3 {
        // create an instance of the Trie
        let mut trie = Trie::new();
        // populate the trie with the names of the files in the .cheat folder
        files
            .iter()
            .for_each(|file_name| trie.insert(&UniCase::new(&file_name.1.name)));
        trie.search(&UniCase::new(&cli_args[2]))
            .iter()
            .for_each(|hit| {
                println!(
                    "{:indent$} {:?}",
                    hit,
                    files.get(&UniCase::new(hit.clone())).unwrap().path,
                    indent = 40
                )
            });
    } else if cli_args[1] == "-e" && cli_args.len() == 3 {
        // for editing a file name
        if !files.contains_key(&UniCase::new(cli_args[2].clone())) {
            // performing a `LOOKUP` in the HashMap for fast search
            // check whether file exists; if not, we create a new one with the prescribed name
            let new_file = Path::new(&cheat_folder).join(&cli_args[2]); // preparing file name
            let result = fs::File::create(&new_file); // creating the file
            match result {
                // checking whether the create function succeeded
                Ok(_) => perform_edit(&config.editor_path.unwrap(), new_file), // if operation succeeds, we open for editing
                Err(error) => eprintln!("An error occurred when creating file: {}", error),
            }
            exit(1);
        }
        let p = &files.get(&UniCase::new(cli_args[2].clone())).unwrap().path; // checking whether the file exists in a CASE INSESNITIVE manner
        perform_edit(&config.editor_path.unwrap(), PathBuf::from(p)); // opening file for edit
    } else if cli_args[1] == "-d" && cli_args.len() == 3 {
        let file_to_delete = &files.get(&UniCase::new(cli_args[2].clone())).unwrap().path;
        let d_res = fs::remove_file(PathBuf::from(file_to_delete));
        match d_res {
            Ok(_) => eprintln!("Deleted the file: {}", &file_to_delete),
            Err(err) => eprintln!("Error deleting file {}: {}", &file_to_delete, err),
        }
    } else if cli_args[1] == "-p" && cli_args.len() == 3 {
        println!("Pushing to git");
        gitops::git::git_add(cheat_folder.to_string_lossy().as_ref(), &cli_args[2]);
    } else {
        // dumps contents of a file onto the screen
        if Path::new(&cheat_folder).join(&cli_args[1]).exists() {
            // checks if file exists
            print!(
                "{}",
                perform_text_dump(&PathBuf::from(
                    &files.get(&UniCase::new(cli_args[1].clone())).unwrap().path
                ))
            ); // if it exists, we get it and unwrap it
        } else {
            eprintln!("The file `{}` does not exist", &cli_args[1].clone())
        } // we print out an error
    }

    Ok(())
}
