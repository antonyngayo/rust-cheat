use std::{io, path::{PathBuf, Path},fs};

use clap::{Parser};

/// simple program to record cheatsheets
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name="cheat")]

struct Arguments {
    #[clap(short, long, value_parser)]
    list: Option<String>,
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: usize
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
    // let args = Arguments::parse();
    let mut files = read_files()?; 
    files.sort();
    for file in files {
        println!("{:indent$} {}", &file.name, &file.path.to_string(), indent=40);
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