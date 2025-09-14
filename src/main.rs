use clap::{Parser, Subcommand};
use std::{
    fs,
    io::{self, stdin, stdout, Write},
    path::{Path, PathBuf},
    process::exit,
};
use trie::tt::Trie;
use unicase::UniCase;

mod configuration;
mod gitops;
mod trie;
mod utils;

use crate::configuration::Config;
use utils::{choose_editor, create_config, perform_edit, perform_text_dump, read_files};

#[derive(Parser)]
#[command(name = "cheat")]
#[command(about = "A cheat sheet management tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Display contents of a cheat sheet (when no subcommand is provided)
    filename: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all cheat sheet files
    #[command(alias = "l")]
    List,
    /// Edit a cheat sheet file
    #[command(alias = "e")]
    Edit {
        /// Name of the file to edit
        filename: String,
    },
    /// Search for cheat sheet files by name
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,
    },
    /// Interactive fuzzy finder
    #[command(alias = "f")]
    Find,
    /// Delete a cheat sheet file
    #[command(alias = "d")]
    Delete {
        /// Name of the file to delete
        filename: String,
    },
    /// Push changes to git repository
    #[command(alias = "p")]
    Push {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    // Initialize configuration and setup
    let mut config = configuration::Config::new();
    let home_dir = dirs::home_dir().unwrap();
    let cheat_folder = Path::new(&home_dir).join(".cheat");
    let files = read_files(&cheat_folder)?;
    let mut selector = vec![];
    let binary_base_path = PathBuf::from("/usr/bin/");
    let binaries = vec!["nano", "vi", "vim", "nvim", "emacs", "ee"];

    // Create and load config file
    match create_config(&config) {
        (true, path) => {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let conf_text: Config = serde_json::from_str(&content).unwrap();
                    config = conf_text;
                }
                Err(_) => eprintln!("The config file is empty"),
            }
            config.config_path = path.to_string_lossy().to_string().to_owned();
            fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
        }
        (false, _) => eprintln!("An error occurred while creating config file"),
    }

    // Setup editor
    match &config.editor_path {
        Some(editor) => {
            if !editor.exists() {
                choose_editor(binaries, &binary_base_path, &mut selector, &mut config)
            }
        }
        None => choose_editor(binaries, &binary_base_path, &mut selector, &mut config),
    }

    // Handle commands
    match cli.command {
        Some(Commands::List) => {
            for file in &files {
                println!(
                    "{:indent$} {}",
                    &file.1.name,
                    &file.1.path.to_string(),
                    indent = 40
                );
            }
        }
        Some(Commands::Search { query }) => {
            let mut trie = Trie::new();
            files
                .iter()
                .for_each(|file_name| trie.insert(&UniCase::new(&file_name.1.name)));
            
            let results = trie.fuzzy_search(&query);
            
            // Filter results to show only high-quality matches (>80% score)
            let high_quality_results: Vec<_> = results.iter()
                .filter(|(_, score)| *score > 80.0)
                .collect();
            
            if high_quality_results.is_empty() {                
                // Show lower quality matches if no high-quality ones exist
                let medium_quality: Vec<_> = results.iter()
                    .filter(|(_, score)| *score > 50.0)
                    .take(5)
                    .collect();
                
                if !medium_quality.is_empty() {
                    for (filename, score) in medium_quality {
                        let file_info = files.get(&UniCase::new(filename.clone())).unwrap();
                        println!(
                            "{:>6.1}% │ {:30} │ {}",
                            score,
                            filename,
                            file_info.path
                        );
                    }
                }
            } else {
                for (filename, _score) in high_quality_results.iter().take(10) {
                    let file_info = files.get(&UniCase::new(filename.clone())).unwrap();
                    println!(
                        "{:30} │ {}",
                        filename,
                        file_info.path
                    );
                }
                
                if high_quality_results.len() > 10 {
                    println!("\n... and {} more high-quality matches", high_quality_results.len() - 10);
                }
            }
        }
        Some(Commands::Find) => {
            let mut trie = Trie::new();
            files
                .iter()
                .for_each(|file_name| trie.insert(&UniCase::new(&file_name.1.name)));
            loop {
                print!("Search> ");
                stdout().flush().unwrap();
                
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        let query = input.trim();
                        if query.is_empty() {
                            continue;
                        }
                        
                        let results = trie.fuzzy_search(query);
                        
                        // Filter for high-quality matches
                        let high_quality_results: Vec<_> = results.iter()
                            .filter(|(_, score)| *score > 80.0)
                            .collect();
                        
                        // Clear previous results (simple approach)
                        print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
                        println!("🔍 Interactive Fuzzy Finder");
                        println!("Type to search, press Enter to select first match, Ctrl+C to exit\n");
                        println!("Search> {}", query);
                        
                        if high_quality_results.is_empty() {
                            // Show medium quality matches as fallback
                            let medium_quality: Vec<_> = results.iter()
                                .filter(|(_, score)| *score > 50.0)
                                .take(5)
                                .collect();
                            
                            if medium_quality.is_empty() {
                                println!("\n❌ No good matches found");
                            } else {
                                println!("\n📁 Found {} medium-quality matches (>50%):", medium_quality.len());
                                for (i, (filename, score)) in medium_quality.iter().enumerate() {
                                    let indicator = if i == 0 { "→" } else { " " };
                                    println!(
                                        "{} {:>6.1}% │ {}",
                                        indicator,
                                        score,
                                        filename
                                    );
                                }
                                
                                if !medium_quality.is_empty() {
                                    print!("\nPress Enter to open '{}' or type new search: ", medium_quality[0].0);
                                    stdout().flush().unwrap();
                                    
                                    let mut selection = String::new();
                                    stdin().read_line(&mut selection).unwrap();
                                    
                                    if selection.trim().is_empty() {
                                        let filename = &medium_quality[0].0;
                                        match files.get(&UniCase::new(filename.clone())) {
                                            Some(file_info) => {
                                                print!("{}", perform_text_dump(&PathBuf::from(&file_info.path)));
                                                break;
                                            }
                                            None => println!("Error: File not found"),
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("\n📁 Found {} high-quality matches (>80%):", high_quality_results.len());
                            for (i, (filename, score)) in high_quality_results.iter().take(5).enumerate() {
                                let indicator = if i == 0 { "→" } else { " " };
                                println!(
                                    "{} {:>6.1}% │ {}",
                                    indicator,
                                    score,
                                    filename
                                );
                            }
                            
                            if high_quality_results.len() > 5 {
                                println!("  ... and {} more", high_quality_results.len() - 5);
                            }
                            
                            // Auto-select first match if user presses enter again
                            print!("\nPress Enter to open '{}' or type new search: ", high_quality_results[0].0);
                            stdout().flush().unwrap();
                            
                            let mut selection = String::new();
                            stdin().read_line(&mut selection).unwrap();
                            
                            if selection.trim().is_empty() {
                                // Open the first match
                                let filename = &high_quality_results[0].0;
                                match files.get(&UniCase::new(filename.clone())) {
                                    Some(file_info) => {
                                        print!("{}", perform_text_dump(&PathBuf::from(&file_info.path)));
                                        break;
                                    }
                                    None => println!("Error: File not found"),
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }
        Some(Commands::Edit { filename }) => {
            if !files.contains_key(&UniCase::new(filename.clone())) {
                let new_file = Path::new(&cheat_folder).join(&filename);
                match fs::File::create(&new_file) {
                    Ok(_) => perform_edit(&config.editor_path.unwrap(), new_file),
                    Err(error) => eprintln!("An error occurred when creating file: {}", error),
                }
                return Ok(());
            }
            let p = &files.get(&UniCase::new(filename)).unwrap().path;
            perform_edit(&config.editor_path.unwrap(), PathBuf::from(p));
        }
        Some(Commands::Delete { filename }) => match files.get(&UniCase::new(filename.clone())) {
            Some(file_info) => {
                let d_res = fs::remove_file(PathBuf::from(&file_info.path));
                match d_res {
                    Ok(_) => eprintln!("Deleted the file: {}", &file_info.path),
                    Err(err) => eprintln!("Error deleting file {}: {}", &file_info.path, err),
                }
            }
            None => eprintln!("The file `{}` does not exist", filename),
        },
        Some(Commands::Push { message }) => {
            println!("Pushing to git");
            gitops::git::git_add(cheat_folder.to_string_lossy().as_ref(), &message);
        }
        None => {
            // Handle direct file access (cheat filename)
            match cli.filename {
                Some(filename) => match files.get(&UniCase::new(filename.clone())) {
                    Some(file_info) => {
                        print!("{}", perform_text_dump(&PathBuf::from(&file_info.path)));
                    }
                    None => eprintln!("The file `{}` does not exist", filename),
                },
                None => {
                    // No command and no filename - show help
                    println!("Use --help to see available commands");
                    exit(1);
                }
            }
        }
    }

    Ok(())
}
