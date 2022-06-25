use std::{io::{self, stdin}, path::{PathBuf, Path},fs, vec, collections::HashMap, process::exit, ffi::{OsStr, OsString}};

use clap::{Parser, Subcommand};
use unicase::UniCase; // this helps with case insensitivity


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

    // global cheat path
    let home_dir = dirs::home_dir().unwrap();
    let cheat_folder = Path::new(&home_dir).join(".cheat");


    let mut files = read_files(cheat_folder)?; // getting the whole list of files in the directory OR an error

    let mut selector = vec![];
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
            match config.editor_path.clone() {
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
                    // println!("You selected {}", editor_index);
                    config.editor_path = Some(selector[editor_index].clone()); // setting the editor
                    println!("{:?}", &config);
                }
            }

            // eprintln!("{}", name);
            let p = &files.get(&UniCase::new(name)).unwrap().path;
            perform_edit(&config.editor_path.unwrap(), PathBuf::from(p));
        },
        Commands::S { term } => {
            eprintln!("{}", term)
        }

    }
    Ok(())
}


fn read_files(cheat_folder: PathBuf) -> Result<HashMap<UniCase<String>, FileNames>, io::Error> { // returning a vector on success or an error on failure 
    // setting home dir as global variable
    let home_dir = dirs::home_dir().unwrap();
    // create a path to .cheat 
    let cheat_path = match &cheat_folder.exists() {
        true => Path::new(&home_dir).join(".cheat"), // setting the path appropriately
        false => {
            let res = fs::create_dir(&cheat_folder);
            match res {
                // if successful, we have created the .chat folder
                Ok(_) => Path::new(&home_dir).join(".cheat"),
                // an error occured and we set an empty path
                Err(_) => { PathBuf::new() }
            }
        }
    };
    let target_folder = std::fs::read_dir(cheat_path)?; // passing the `.cheat` psth to be read and enumerated
    let mut res = HashMap::new(); // declaring the vec object

    // name to be used as key in the hashmap

    for (_, file) in target_folder.enumerate(){  
        let hname = file.as_ref().unwrap().path().to_string_lossy().split("/").last().unwrap().to_string();
        res.insert(UniCase::new(hname), FileNames::new(file.unwrap().path())); // pushing the paths into the vector
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

fn perform_edit(binary: &PathBuf, file_path: PathBuf) {
    let w = binary.to_string_lossy().to_string().to_owned();
    let bin = OsString::from(w);

    std::process::Command::new(bin).arg(file_path).output().expect("An error occured while performing edit");
}