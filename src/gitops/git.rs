use std::process::Command;

#[allow(unused)]
pub fn git_add(cheat_dir: &str, commit_msg: &str){
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(cheat_dir).arg("fetch").arg("--prune").arg("origin");
    eprintln!("[INFO] Fetching changes from remote");
    match cmd.output() {
        Ok(_) => {
            // merge if there are any changes
            cmd.arg("-C").arg(cheat_dir).arg("merge").arg("origin/master");
            eprintln!("[INFO] Merging changes from remote");
            match cmd.output() {
                Ok(_) => {
                    cmd.arg("-C").arg(cheat_dir).arg("add").arg("-A");
                    eprintln!("[INFO] Adding files to git");
                    match cmd.output() {
                        Ok(_) => {
                            cmd.arg("-C").arg(cheat_dir).arg("commit").arg("-m").arg(commit_msg);
                            eprintln!("[INFO] Committing changes");
                            match cmd.output() {
                                Ok(_) => {
                                    cmd.arg("-C").arg(cheat_dir).arg("push").arg("origin").arg("master");
                                    eprintln!("[INFO] Pushing changes to remote");
                                    match cmd.output() {
                                        Ok(_) => println!("[INFO] Successfully pushed to remote"),
                                        Err(err) => println!("[ERROR] An error occured while pushing to remote: {err}")
                                    }
                                },
                                Err(err) => println!("[ERROR] An error occured while committing files: {err}")
                            }
                        },
                        Err(err) => println!("[ERROR] An error occured while adding files: {err}")
                    }
                },
                Err(err) => println!("[ERROR] An error occured while merging repo: {err}")
            }
        }
        Err(err) => println!("[ERROR] An error occured while fetching files from origin master: {err}")
    }
}