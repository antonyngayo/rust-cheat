use std::process::Command;
use std::path::Path;

#[allow(unused)]
pub fn git_add(cheat_dir: &str, commit_msg: &str){
    let mut cmd = Command::new("/usr/bin/git");
    let p = cheat_dir.to_string() + "/";
    let path = Path::new(&p);
    if path.exists() {
        cmd.args(&["-C", &path.to_string_lossy(),"fetch", "--prune", "origin", "master"]);
        eprintln!("[INFO] Fetching changes from remote from origin/master : Cheat Path: {:?}", path.display());
        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("[ERROR] An error occured while fetching files from origin master: {:?}", output.stderr);
                    return;
                }
                let mut cmd_merge = Command::new("/usr/bin/git");
                cmd_merge.arg("-C").arg(path).arg("merge").arg("origin/master").arg("--allow-unrelated-histories");
                eprintln!("[INFO] Merging changes from remote: cheat path: {}", path.display());
                match cmd_merge.output() {
                    Ok(output) => {
                        if !output.status.success() {
                            eprintln!("[ERROR] An error occured while merging repo: {:?}", output.stderr);
                            return;
                        }
                        let mut cmd_add = Command::new("/usr/bin/git");
                        cmd_add.arg("-C").arg(path).arg("add").arg(".");
                        eprintln!("[INFO] Adding files to git: cheat path: {}", path.display());
                        match cmd_add.output() {
                            Ok(output) => {
                                if !output.status.success() {
                                    eprintln!("[ERROR] An error occured while adding files: {:?}", output.stderr);
                                    return;
                                }
                                let mut cmd_commit = Command::new("/usr/bin/git");
                                cmd_commit.arg("-C").arg(path).arg("commit").arg("-m").arg(commit_msg);
                                eprintln!("[INFO] Committing changes: cheat path: {}", path.display());
                                match cmd_commit.output() {
                                    Ok(message) => {
                                        if !message.status.success() {
                                            eprintln!("[ERROR] An error occured while committing files: {:?}", message.stderr);
                                            return;
                                        }
                                        let mut cmd_push = Command::new("/usr/bin/git");
                                        cmd_push.arg("-C").arg(path).arg("push").arg("origin").arg("master");
                                        eprintln!("[INFO] Pushing changes to remote: cheat path: {}", path.display());
                                        match cmd_push.output() {
                                            Ok(message) => {
                                                if !message.status.success() {
                                                    eprintln!("[ERROR] An error occured while pushing files: {:?}", message.stderr);
                                                    return;
                                                }
                                                eprintln!("[INFO] Successfully pushed to remote: {}", path.display())
                                            },
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
    
}