use std::{io::{self, stdin}, path::{PathBuf, Path},fs, vec};

use clap::{Parser, Subcommand};


mod configuration;

/// simple program to record cheatsheets
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name="cheat")]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
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

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)] // this allows the structure to be Order-able and
struct FileNames {
    name: String,
    path: String
}
impl FileNames {
    fn new(p: PathBuf) -> Self {
        Self { name: p.to_string_lossy().split("/").last().unwrap().to_string(), path: p.to_string_lossy().to_string() }
    }
}

fn main() -> Result<(), io::Error>{
    let mut config = configuration::Config::new();
    match create_config(){
        (true, path) => {
            config.config_path = path.to_string_lossy().to_string().to_owned(); // setting the config path to be written to disk soon
        },
        (false, _) => eprintln!("An error occured while creating config file")
    }

    let binary_base_path = PathBuf::from("/usr/bin/");
    let binaries = vec!["nano", "vi","vim", "nvim"];
    let args = Cli::parse();
    let mut files = read_files()?; // getting the whole list of files in the directory OR an error
    let mut selector = vec![];
    match args.command {
        Commands::L {  } => { 
            files.sort();
            for file in files {
                println!("{:indent$} {}", &file.name, &file.path.to_string(), indent=40);
            }
        },
        Commands::D { name } => {
            eprintln!("{}", name)
        },
        Commands::E { name } => {
            // if !files.contains(name) {
            //     eprintln!("The file does not exist");
            // }
            match config.editor_path {
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
                    println!("You selected {}", editor_index);
                    config.editor_path = Some(selector[editor_index].clone()); // setting the editor
                    println!("{:?}", &config);
                }
            }
            eprintln!("{}", name);
        },
        Commands::S { term } => {
            eprintln!("{}", term)
        }

    }
    Ok(())
}


fn read_files() -> Result<Vec<FileNames>, io::Error> { // returning a vector on success or an error on failure 
    // setting home dir as global variable
    let home_dir = dirs::home_dir().unwrap();
    // create a path to .cheat 
    let cheat_path = match Path::new(&home_dir).join(".cheat").exists() {
        true => Path::new(&home_dir).join(".cheat"), // setting the path appropriately
        false => {
            let res = fs::create_dir(Path::new(&home_dir).join(".cheat"));
            match res {
                // if successful, we have created the .chat folder
                Ok(_) => Path::new(&home_dir).join(".cheat"),
                // an error occured and we set an empty path
                Err(_) => { PathBuf::new() }
            }
        }
    };
    let target_folder = std::fs::read_dir(cheat_path)?; // passing the `.cheat` psth to be read and enumerated
    let mut res = vec![]; // declaring the vec object
    for file in target_folder{  
        res.push(FileNames::new(file?.path())); // pushing the paths into the vector
    }
    return Ok(res);
}


// create a config file

fn create_config() -> (bool, PathBuf) {
    // creating a config file for the cheat binary 
    let home_dir = dirs::home_dir().unwrap();
    match Path::new(&home_dir).join(".cheat.config").exists(){
        true => { return (true, Path::new(&home_dir).join(".cheat.json")) },
        false => {
            let path =  Path::new(&home_dir).join(".cheat.json");
            let res = fs::File::create(path);
            match res {
                Ok(_) => return (true, Path::new(&home_dir).join(".cheat.json")),
                Err(_) => return (false, PathBuf::new())
            }
        },
    };
}

fn check_for_editor(binaries: Vec<&str>, binary_base_path: &PathBuf, selector: &mut Vec<PathBuf>){
    for bin in binaries {
        match Path::new(&binary_base_path).join(bin).exists() {
            true => {
                selector.push(binary_base_path.join(bin));
            },
            false => {continue;}
        }
    }
}