use clap::{Args, Parser, Subcommand};
use colored::*;
use difference::Changeset;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct DifferArgs {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Args, Debug)]
struct DiffArgs {
    // Files to compare
    files: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Diff(DiffArgs),
}

fn print_diff(v1: &str, v2: &str) {
    let Changeset { diffs, .. } = Changeset::new(v1, v2, "\n");
    for diff_cache in &diffs {
        match diff_cache {
            difference::Difference::Same(part) => println!("{}", part),
            difference::Difference::Add(part) => part
                .split("\n")
                .for_each(|l| println!("{}", format!("+ {}", l).green())),
            difference::Difference::Rem(part) => part
                .split("\n")
                .for_each(|l| println!("{}", format!("- {}", l).red())),
        }
    }
}

/// Usage: diff f1 f2
fn diff_files(files: Vec<String>) {
    if files.len() != 2 {
        panic!("Only two files are supported");
    }
    let file_contents = files
        .iter()
        .map(|file| std::fs::read_to_string(file).unwrap())
        .collect::<Vec<String>>();
    print_diff(&file_contents[0], &file_contents[1]);
}

fn main() {
    let args = DifferArgs::parse();
    match args.cmd {
        Some(Cmd::Diff(args)) => diff_files(args.files),
        None => println!("No subcommand was used"),
    }
}
