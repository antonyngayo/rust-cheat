use std::process::Command;
use std::path::Path;

#[allow(unused)]
pub fn git_add(cheat_dir: &str, commit_msg: &str){
    let mut cmd = Command::new("/usr/bin/git");
    let p = cheat_dir.to_string() + "/";
    let path = Path::new(&p);
    if path.exists() {
        cmd.args(&["-C", &path.to_string_lossy(),"fetch", "--prune", "origin", "master"]);
        eprintln!("[INFO] Fetching changes from remote from origin/master");
        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("[ERROR] An error occured while fetching files from origin master: {:?}", output.stderr);
                    return;
                }
                let mut cmd_merge = Command::new("/usr/bin/git");
                cmd_merge.arg("-C").arg(path).arg("merge").arg("origin/master").arg("--allow-unrelated-histories");
                eprintln!("[INFO] Merging changes from remote");
                match cmd_merge.output() {
                    Ok(output) => {
                        if !output.status.success() {
                            let msg = String::from_utf8(output.stderr.to_vec()).unwrap();
                            eprintln!("[ERROR] An error occured while merging files: {msg}");
                            return;
                        }
                        let mut cmd_add = Command::new("/usr/bin/git");
                        cmd_add.arg("-C").arg(path).arg("add").arg(".");
                        eprintln!("[INFO] Adding files to git");
                        match cmd_add.output() {
                            Ok(output) => {
                                if !output.status.success() {
                                    let msg = String::from_utf8(output.stderr.to_vec()).unwrap();
                                    eprintln!("[ERROR] An error occured while adding files: {msg}");
                                    return;
                                }
                                let mut cmd_commit = Command::new("/usr/bin/git");
                                cmd_commit.arg("-C").arg(path).arg("commit").arg("-m").arg(commit_msg);
                                eprintln!("[INFO] Committing changes");
                                match cmd_commit.output() {
                                    Ok(message) => {
                                        if !message.status.success() {
                                            let msg = String::from_utf8(message.stderr.to_vec()).unwrap();
                                            if message.stderr.len() != 0 {
                                                eprintln!("[ERROR] An error occured while [1. committing] files: {:?}", message.stderr);
                                                return;
                                            }
                                            if String::from_utf8(message.stdout.to_vec()).unwrap().replace("\n","").contains("nothing to commit") {
                                                eprintln!("[INFO] Nothing to commit");
                                                return;
                                            }
                                            eprintln!("[ERROR] An error occured while committing files");
                                            return;
                                        }
                                        let mut cmd_push = Command::new("/usr/bin/git");
                                        cmd_push.arg("-C").arg(path).arg("push").arg("origin").arg("master");
                                        eprintln!("[INFO] Pushing changes to remote");
                                        match cmd_push.output() {
                                            Ok(message) => {
                                                if !message.status.success() {
                                                    let msg = String::from_utf8(message.stderr.to_vec()).unwrap();
                                                    eprintln!("[ERROR] An error occured while pushing files: {msg}");
                                                    return;
                                                }
                                                eprintln!("[INFO] Successfully pushed to remote")
                                            },
                                            Err(err) => println!("[ERROR] An error occured while pushing to remote: {err}")
                                        }
                                    },
                                    Err(err) => println!("[ERROR] Nothing to commit in the branch: {err}")
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