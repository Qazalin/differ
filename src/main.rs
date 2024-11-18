use clap::{Args, Parser, Subcommand};

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
    let diffable = usize::min(v1.len(), v2.len());
    let good_lines = Vec::from_iter(v1.lines());
    for (i, compare) in v2.lines().enumerate() {
        let good = good_lines[i];
        match compare == good {
            true => println!("{good}"),
            false => {
                println!("\x1b[31m- {}\x1b[0m", good);
                println!("\x1b[32m+ {}\x1b[0m", compare);
            }
        }
    }
    if diffable < usize::max(v1.len(), v2.len()) {
        println!("\x1b[33mdiffed less lines\x1b[0m");
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
