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

#[derive(Args, Debug)]
struct DiffArgs {
    // Files to compare
    files: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    New(NewArgs),
    Diff(DiffArgs),
    Clean,
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

fn print_diff(v1: &str, v2: &str) {
    let Changeset { diffs, .. } = Changeset::new(v1, v2, "\n");
    for diff_cache in &diffs {
        match diff_cache {
            // TODO write this part yourself
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

fn diff_cache() {
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
        print_diff(&args[0].content, &args[1].content);
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
        Some(Cmd::New(args)) => new(&args),
        Some(Cmd::Diff(args)) => match args.files {
            Some(files) => diff_files(files),
            None => diff_cache(),
        },
        Some(Cmd::Clean) => {
            clear_differ_tempfiles();
            clear_differ_files();
        }
        None => println!("No subcommand was used"),
    }
}

fn clear_differ_files() {
    let fns = ["k0", "k1"];
    let pwd = std::env::current_dir().unwrap();
    if let Ok(entries) = std::fs::read_dir(pwd) {
        for entry in entries {
            if let Ok(entry) = entry {
                if fns.contains(&entry.path().file_name().unwrap().to_str().unwrap()) {
                    std::fs::remove_file(entry.path()).unwrap();
                }
            }
        }
    }
}
fn clear_differ_tempfiles() {
    let differ_path = get_differ_path();
    let differ_loc = Path::new(&differ_path);
    if differ_loc.exists() {
        std::fs::remove_file(differ_loc).unwrap();
    }
}
/// NOTE: Use one thread: cargo test -- --test-threads=1
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        clear_differ_tempfiles();
        new(&NewArgs {
            content: "a\nb\nc\nd\ne".to_string(),
            id: "test".to_string(),
            variant: "b".to_string(),
        });
        let new_args = get_saved_args();
        assert_eq!(new_args.len(), 1);
        assert_eq!(new_args[0].content, "a\nb\nc\nd\ne");
    }
    #[test]
    fn test_diff_cache() {
        clear_differ_tempfiles();
        let args = vec![
            NewArgs {
                content: "a\nb\nc".to_string(),
                id: "test".to_string(),
                variant: "a".to_string(),
            },
            NewArgs {
                content: "a\nb\nc\nd".to_string(),
                id: "test".to_string(),
                variant: "b".to_string(),
            },
        ];
        let differ_path = get_differ_path();
        let differ_loc = Path::new(&differ_path);
        let encoded_args: Vec<u8> = bincode::serialize(&args).unwrap();
        std::fs::write(Path::new(&differ_loc), encoded_args).unwrap();
        diff_cache();
    }
}
