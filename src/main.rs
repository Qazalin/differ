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

fn temp(x: &str) -> String {
    let temp_dir = tempfile::tempdir().unwrap().into_path();
    let temp_path: PathBuf = temp_dir.join(x);
    return temp_path.to_str().unwrap().to_string();
}

fn new(args: &NewArgs) {
    let file_loc = temp(&format!("{}_{}.df", args.id, args.variant));
    let file = Path::new(&file_loc);
    std::fs::write(file, args.clone().content).expect("Unable to write file");
    println!("Created file at {}", file_loc);

    let differ_path = std::env::var("HOME").unwrap() + "/.differ";
    let differ_loc = Path::new(&differ_path);
    if !differ_loc.exists() {
        std::fs::write(differ_loc, "").expect("Unable to write file");
    }
    let differ_content = std::fs::read_to_string(differ_loc).expect("Unable to read file");
    let mut all_args: Vec<NewArgs> = vec![];
    println!("Differ content: {}", differ_content);
    if differ_content.len() != 0 {
        all_args = bincode::deserialize(&differ_content.as_bytes()).unwrap();
        println!("All args: {:?}", all_args);
    }
    all_args.push(args.clone());
    let encoded_args: Vec<u8> = bincode::serialize(&all_args).unwrap();
    std::fs::write(differ_loc, encoded_args).expect("Unable to write file");
}

fn diff() {
    todo!();
}

fn main() {
    let args = DifferArgs::parse();
    match args.cmd {
        Some(Cmd::New(args)) => new(&args),
        Some(Cmd::Diff) => diff(),
        None => println!("No subcommand was used"),
    }
}
