use std::process::Command;

#[allow(unused)]
pub fn git_add(cheat_dir: &str, commit_msg: &str){
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(cheat_dir).arg("add").arg(".");
    let res = cmd.output();
    match res {
        Ok(_) => {
            let mut cmd = Command::new("git");
            cmd.arg("-C").arg(cheat_dir).arg("commit").arg("-m").arg(commit_msg);
            let res = cmd.output();
            match res {
                Ok(_) => {
                    let mut cmd = Command::new("git");
                    cmd.arg("-C").arg(cheat_dir).arg("push");
                    let res = cmd.output();
                    match res {
                        Ok(_) => println!("Successfully pushed to remote"),
                        Err(_) => println!("An error occured while pushing to remote")
                    }
                
                },
                Err(_) => println!("An error occured while committing files")
            }
        }
        Err(_) => println!("An error occured while adding files to git")
    }
}