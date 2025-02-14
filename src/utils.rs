use std::{collections::HashMap, fs::{self, File}, io::{self, stdin, BufReader, Read}, path::{Path, PathBuf}};

use unicase::UniCase;

use crate::configuration::Config;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)] // this allows the structure to be Order-able and
pub struct FileNames {
    pub name: String,
    pub path: String
}
impl FileNames {
    pub fn new(p: PathBuf) -> Self {
        Self { name: p.to_string_lossy().split("/").last().unwrap().to_string(), path: p.to_string_lossy().to_string() }
    }
}
pub fn read_files(cheat_folder: &PathBuf) -> Result<HashMap<UniCase<String>, FileNames>, io::Error> { // returning a vector on success or an error on failure 
    // setting home dir as global variable
    let home_dir = dirs::home_dir().unwrap();
    // create a path to .cheat 
    let cheat_path = match &cheat_folder.exists() {
        true => Path::new(&home_dir).join(".cheat"), // setting the path appropriately
        false => {
            let res = fs::create_dir(cheat_folder);
            match res {
                // if successful, we have created the .chat folder
                Ok(_) => Path::new(&home_dir).join(".cheat"),
                // an error occured and we set an empty path
                Err(_) => PathBuf::new()
            }
        }
    };
    let target_folder = std::fs::read_dir(cheat_path)?; // passing the `.cheat` psth to be read and enumerated
    let mut res = HashMap::new(); // declaring the vec object
    // name to be used as key in the hashmap
    for file in target_folder{  
        let hname = file.as_ref().unwrap().path().to_string_lossy().split("/").last().unwrap().to_string();
        res.insert(UniCase::new(hname), FileNames::new(file.unwrap().path())); // pushing the paths into the vector
    }
    Ok(res)
}


// create a config file
pub fn create_config(config: &Config) -> (bool, PathBuf) {
    // creating a config file for the cheat binary 
    let home_dir = dirs::home_dir().unwrap();
    match Path::new(&home_dir).join(".cheat.json").exists(){
        true => (true, Path::new(&home_dir).join(".cheat.json")),
        false => {
            let path =  Path::new(&home_dir).join(".cheat.json");
            let res = fs::File::create(&path);
            // filling the config file with the initial configurations so the file has something
            fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
            match res {
                Ok(_) => (true, Path::new(&home_dir).join(".cheat.json")),
                Err(_) => (false, PathBuf::new())
            }
        },
    }
}

// checks whether the editor binary exists in the OS
pub fn check_for_editor(binaries: Vec<&str>, binary_base_path: &PathBuf, selector: &mut Vec<PathBuf>){
    for bin in binaries {
        match Path::new(&binary_base_path).join(bin).exists() {
            true => selector.push(binary_base_path.join(bin)),
            false => continue
        }
    }
}
// open the files using the chosen editor; bash does the terminal spawning under the hood
pub fn perform_edit(binary: &PathBuf, file_path: PathBuf) {
    //// when starting the SHELL, you need to spawn a binary then wait for it to finish execution. That is why we are using `spawn` and `wait` 
    let _ = std::process::Command::new(binary).arg(file_path.to_string_lossy().as_ref()).spawn().expect("An error occured while performing edit").wait();
}
// open the files using the chosen editor; bash does the terminal spawning under the hood
pub fn perform_text_dump(file_path: &PathBuf) -> String {
    let mut buffer = String::new();
    let mut buf_reader = BufReader::new(File::open(file_path).expect("Could not open file"));
    buf_reader.read_to_string(&mut buffer).expect("Could not stream");
    buffer
}

// allows user to choose the editor of their choice and saves it in the config file
pub fn choose_editor(binaries: Vec<&str>, binary_base_path: &PathBuf, selector: &mut Vec<PathBuf>, config: &mut Config) {
    let mut editor_selection = String::new();
    check_for_editor(binaries, binary_base_path, selector);
    println!("Please select an editor");
    for (index, editor) in selector.iter().enumerate() {
        println!("{}) {:?} ", index + 1, editor); // lists out all the editors found in the system
    }
    let mut editor_index = 0; // initializing the variable
    stdin().read_line(&mut editor_selection).ok(); // getting input from user
    match editor_selection.trim().parse::<usize>() {
        Ok(num) => { if num < selector.len() + 1 { editor_index = num - 1 } },
        Err(e) => eprintln!("An error occurred during casting: {}", e)
    }
    config.editor_path = Some(selector[editor_index].clone()); // setting the editor
    fs::write(&config.config_path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::perform_text_dump;
    
    #[test]
    fn read_file(){
        let data = perform_text_dump(&PathBuf::from("test.txt"));
        dbg!(data.len());
        assert!(data.len() == 867);
    }
}


