pub mod git {
    use std::io::Error;
    use std::process::Command;
    use utils::utils;

    pub fn git_commit_and_push(repo_path: &String, msg: String) -> Result<(), ()> {
        // TODO: See how to chain these function calls
        git_add(repo_path).unwrap();
        git_commit(repo_path, msg).unwrap();
        git_push(repo_path).unwrap();
        Ok(())
    }

    fn git() -> String {
        if utils::is_program_in_path("git") {
            String::from("git")
        } else {
            panic!("Cannot locate executable - git - on your system")
        }
    }

    fn git_add(repo_path: &String) -> Result<(), Error> {
        match Command::new(git())
            .arg(format!("--git-dir={}/.git/", repo_path))
            .arg(format!("--work-tree={}", repo_path))
            .arg("add")
            .arg("-A")
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Could not stage files to repo at [{}]: {}", repo_path, e);
                Err(e)
            }
        }
    }

    fn git_commit(repo_path: &String, msg: String) -> Result<(), Error> {
        match Command::new(git())
            .arg(format!("--git-dir={}/.git/", repo_path))
            .arg(format!("--work-tree={}", repo_path))
            .arg("commit")
            .arg("-m")
            .arg(msg)
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!(
                    "Could not commit new idea to repo at [{}]: {}",
                    repo_path, e
                );
                Err(e)
            }
        }
    }

    fn git_push(repo_path: &String) -> Result<(), Error> {
        match Command::new(git())
            .arg(format!("--git-dir={}/.git/", repo_path))
            .arg(format!("--work-tree={}", repo_path))
            .arg("push")
            .arg("origin")
            .arg("master")
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!(
                    "Could not push commit to remote 'origin' and \
                     branch 'master' in repo at [{}]: {}",
                    repo_path, e
                );
                Err(e)
            }
        }
    }
}
