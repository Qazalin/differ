use clap::{Args, Parser, Subcommand};
use colored::*;
use difference::Changeset;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    return dir_path.join(format!("{}.bin", scope));
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

    let mut override_idx = None;
    let mut seen: HashMap<String, usize> = HashMap::new();
    for (arg, idx) in all_args.iter().zip(0..) {
        if arg.id == args.id && arg.variant == args.variant {
            override_idx = Some(idx);
        } else if let Some(seen_count) = seen.get(&arg.id) {
            if seen_count == &2 {
                panic!("id {} can't have more than two variants", arg.id);
            }
        }
        let count = seen.entry(arg.id.clone()).or_insert(0);
        *count += 1;
    }
    match override_idx {
        Some(i) => all_args[i] = args.clone(),
        None => all_args.push(args.clone()),
    }
    let encoded_args: Vec<u8> = bincode::serialize(&all_args).unwrap();
    std::fs::write(Path::new(&differ_path), encoded_args).unwrap();
}

fn diff() {
    let args = get_saved_args();
    let grouped_args: HashMap<_, Vec<_>> = args
        .into_iter()
        .group_by(|arg| arg.id.clone())
        .into_iter()
        .map(|(k, group)| (k, group.collect()))
        .collect();

    for (name, args) in grouped_args {
        println!(
            "-- ID: {}. Variants {}, {}",
            name, args[0].variant, args[1].variant
        );
        let Changeset { diffs, .. } = Changeset::new(&args[0].content, &args[1].content, "\n");
        for diff in &diffs {
            match diff {
                difference::Difference::Same(part) => println!("  {}", part),
                difference::Difference::Add(part) => println!("{}", format!("+ {}", part).green()),
                difference::Difference::Rem(part) => println!("{}", format!("- {}", part).red()),
            }
        }
    }
}

fn main() {
    let args = DifferArgs::parse();
    match args.cmd {
        Some(Cmd::New(args)) => new(&args),
        Some(Cmd::Diff) => diff(),
        None => println!("No subcommand was used"),
    }
}
