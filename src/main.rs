#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::env;
use std::fs;
use std::io;
use std::io::{Error, Write};
use std::process::Command;

#[macro_use]
extern crate text_io;

#[macro_use]
extern crate clap;

use clap::ArgMatches;
use clap::{App, Arg};
use file_handler::file_handler as fh;
use file_handler::file_handler::ConfigFile::*;
use git::git::git_commit_and_push;

mod file_handler;
mod git;

fn main() {
    let cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        .arg(
            Arg::with_name("clear-repo")
                .long("clear-repo")
                .help("Clear the stored path to your idea repo"),
        )
        .arg(
            Arg::with_name("clear-editor")
                .long("clear-editor")
                .help("Clear the stored path to your idea editor"),
        )
        .get_matches();

    if cli_flags.is_present("clear-repo") {
        match fh::rm_file(Repo.name()) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    if cli_flags.is_present("clear-editor") {
        match fh::rm_file(Editor.name()) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    let repo_path: String = match fh::read_from_config(Repo.name()) {
        Ok(file_path) => file_path,
        Err(_) => {
            display_first_time_setup_banner();
            if !fh::path_exists(&fh::config_location()) {
                match fs::create_dir_all(&fh::config_location()) {
                    Ok(_) => {}
                    Err(_) => panic!(
                        "Could not create dir at {} to store necessary config",
                        fh::config_location()
                    ),
                }
            }

            print!("Absolute path to your idea repo: ");
            io::stdout().flush().unwrap();
            let input_path: String = read!();
            let copy_input_path: String = input_path.clone();

            match fh::write_to_config(Repo.name(), input_path) {
                Ok(_) => copy_input_path,
                Err(e) => panic!("Unable to write your repo path to disk: {}", e),
            }
        }
    };

    let editor_path_from_env: Option<String> = ::env::var("EDITOR").ok();

    fn editor_path_from_config() -> String {
        match fh::read_from_config(Editor.name()) {
            Ok(file_path) => file_path,
            Err(_) => {
                println!("What editor do you want to use for writing down your ideas?");
                println!("1) vim (/usr/bin/vim)");
                println!("2) nano (/usr/bin/nano)");
                println!("3) Other (provide path to binary)");
                println!();
                print!("Alternative: ");
                io::stdout().flush().unwrap();

                let input_choice: String = read!();
                // Cast to int to be able to match
                let editor_choice: u32 = input_choice.parse::<u32>().unwrap();
                let input_path: String = match editor_choice {
                    1 => s("/usr/bin/vim"),
                    2 => s("/usr/bin/nano"),
                    3 => {
                        print!("Path to editor binary: ");
                        io::stdout().flush().unwrap();
                        let editor_bin_path: String = read!();
                        editor_bin_path
                    }
                    _ => {
                        // TODO: Do not fall back, ask user again for options
                        println!("Invalid option, falling back to vim");
                        s("/usr/bin/vim")
                    }
                };

                if !fh::path_exists(&input_path) {
                    panic!("Invalid editor path");
                }

                let copy_input_path: String = input_path.clone();
                match fh::write_to_config(Editor.name(), input_path) {
                    Ok(_) => copy_input_path,
                    Err(e) => panic!("Unable to write your editor path to disk: {}", e),
                }
            }
        }
    }

    let editor_bin_with_args: (String, Vec<String>) = editor_path_from_env.and_then(|p| find_bin(p)).or_else(|| find_bin(editor_path_from_config())).unwrap();

    let commit_msg: String = get_commit_msg();
    let readme_path: String = format!("{}/README.md", repo_path);

    match open_editor(&editor_bin_with_args.0, &editor_bin_with_args.1, &readme_path) {
        Ok(_) => {
            let _ = git_commit_and_push(&repo_path, commit_msg);
        }
        Err(e) => panic!("Could not open editor at path {}: {}", editor_bin_with_args.0, e),
    };
}

fn display_first_time_setup_banner() {
    println!();
    println!("##########################################################");
    println!("####                 First Time Setup                 ####");
    println!("##########################################################");
    println!();
    println!("This tool requires you to have a repository with a README.md");
    println!("in the root folder. The markdown file is where your ideas will");
    println!("be stored.");
    println!();
}

fn get_commit_msg() -> String {
    println!("Idea commit subject: ");
    let mut input = String::new();
    // The library text_io doesn't read input
    // if it has any whitespace in it
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn find_bin(editor: String) -> Option<(String, Vec<String>)> {
    let mut bin_editor_with_args = editor.split(' ');
    let bin_name = bin_editor_with_args.next().unwrap();

    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&bin_name);
            if full_path.is_file() {
                full_path.to_str().map(|s|String::from(s))
            } else {
                None
            }
        }).next()
    }).map(|p| (p, bin_editor_with_args.map(|s| String::from(s)).collect()))
}

fn open_editor(bin_path: &String, args: &Vec<String>, file_path: &String) -> Result<(), Error> {
    let mut command: Command = args.iter().fold(Command::new(bin_path), |mut c, i| {
        c.arg(i);
        c
    });
    match command.arg(file_path).status() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "Unable to open file [{}] with editor binary at [{}]: {}",
                file_path, bin_path, e
            );
            Err(e)
        }
    }
}

/*
 * Helpers
 */

fn s(string: &str) -> String {
    string.to_owned()
}

fn str(string: &str) -> String {
    String::from(string)
}
