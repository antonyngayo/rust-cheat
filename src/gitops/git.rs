use std::process::Command;

#[allow(unused)]
pub fn git_add(cheat_dir: &str, commit_msg: &str){
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(cheat_dir).arg("add").arg(".");
    eprintln!("[INFO] Adding files to git");
    match cmd.output() {
        Ok(_) => {
            cmd.arg("-C").arg(cheat_dir).arg("fetch").arg("--prune").arg("origin");
            eprintln!("[INFO] Fetching changes from remote");
            match cmd.output() {
                Ok(_) => {
                    cmd.arg("-C").arg(cheat_dir).arg("pull").arg("--rebase").arg("origin");
                    eprintln!("[INFO] Pulling changes from master");
                    match cmd.output() {
                        Ok(_) => {
                            cmd.arg("-C").arg(cheat_dir).arg("commit").arg("-m").arg(commit_msg);
                            eprintln!("[INFO] Committing changes");
                            match cmd.output() {
                                Ok(_) => {
                                    cmd.arg("-C").arg(cheat_dir).arg("push");
                                    eprintln!("[INFO] Pushing changes to remote");
                                    match cmd.output() {
                                        Ok(_) => println!("[INFO] Successfully pushed to remote"),
                                        Err(err) => println!("[ERROR] An error occured while pushing to remote: {err}")
                                    }
                                },
                                Err(err) => println!("[ERROR] An error occured while committing files: {err}")
                            }
                        },
                        Err(err) => println!("[ERROR] An error occured while fetching from remote: {err}")
                    }
                },
                Err(err) => println!("[ERROR] An error occured while fetching from remote: {err}")
            }
        }
        Err(err) => println!("[ERROR] An error occured while adding files to git: {err}")
    }
}