use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct DifferArgs {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Args, Debug, Serialize, Deserialize, Clone)]
struct NewArgs {
    content: String,
    /// id is shared between all the variants
    id: String,
    variant: String, // TODO should variant be derived?
}
#[derive(Subcommand, Debug)]
enum Cmd {
    New(NewArgs),
    Diff,
}

fn get_differ_path() -> PathBuf {
    let pwd = std::env::current_dir().unwrap();
    let scope = pwd.file_name().unwrap().to_str().unwrap();
    let dir_path = PathBuf::from("/tmp/.differ");
    if !dir_path.exists() {
        std::fs::create_dir(&dir_path).unwrap();
    }

    let differ_path = dir_path.join(format!("{}.bin", scope));
    return differ_path;
}

fn get_saved_args() -> Vec<NewArgs> {
    let differ_path = get_differ_path();
    let differ_loc = Path::new(&differ_path);
    if !differ_loc.exists() {
        std::fs::write(differ_loc, "").unwrap();
    }
    let differ_content = std::fs::read_to_string(differ_loc).unwrap();
    let mut all_args: Vec<NewArgs> = vec![];
    if differ_content.len() != 0 {
        all_args = bincode::deserialize(&differ_content.as_bytes()).unwrap();
    }
    return all_args;
}

fn new(args: &NewArgs) {
    let mut all_args = get_saved_args();
    let differ_path = get_differ_path();
    all_args.push(args.clone());
    let encoded_args: Vec<u8> = bincode::serialize(&all_args).unwrap();
    std::fs::write(Path::new(&differ_path), encoded_args).unwrap();
}

fn diff() {
    let all_args = get_saved_args();
    println!("{:?}", all_args);
}

fn main() {
    let args = DifferArgs::parse();
    match args.cmd {
        Some(Cmd::New(args)) => new(&args),
        Some(Cmd::Diff) => diff(),
        None => println!("No subcommand was used"),
    }
}
