use clap::{Args, Parser, Subcommand};
use difference::{Changeset, Difference};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
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

    let mut found = None;
    for (arg, idx) in all_args.iter().zip(0..) {
        if arg.id == args.id && arg.variant == args.variant {
            found = Some(idx);
        }
    }
    if let Some(idx) = found {
        all_args[idx] = args.clone();
    } else {
        all_args.push(args.clone());
    }

    let encoded_args: Vec<u8> = bincode::serialize(&all_args).unwrap();
    std::fs::write(Path::new(&differ_path), encoded_args).unwrap();
}

fn display_diff(text1: &str, text2: &str) {
    let Changeset { diffs, .. } = Changeset::new(text1, text2, "\n");
    let mut t = term::stdout().unwrap();

    for diff in &diffs {
        match diff {
            Difference::Same(ref x) => {
                t.reset().unwrap();
                writeln!(t, " {}", x);
            }
            Difference::Add(ref x) => {
                t.fg(term::color::GREEN).unwrap();
                writeln!(t, "+{}", x);
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED).unwrap();
                writeln!(t, "-{}", x);
            }
        }
    }

    t.reset().unwrap();
    t.flush().unwrap();
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
        println!("Group: {}", name);

        for i in 0..args.len() {
            for j in i + 1..args.len() {
                println!(
                    "Differences between Variant {} and Variant {}:",
                    args[i].variant, args[j].variant
                );
                display_diff(&args[i].content, &args[j].content);
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
