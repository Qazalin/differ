use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct DifferArgs {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Args, Debug)]
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

fn new(id: String, variant: String, content: String) {
    let file_loc = temp(&format!("{}_{}.df", id, variant));
    let file = Path::new(&file_loc);
    std::fs::write(file, content).expect("Unable to write file");
    println!("Created file at {}", file_loc);

    let differ_log = Path::new("/tmp/differ_log");
    println!("Differ log: {}", differ_log.display());
    if !differ_log.exists() {
        std::fs::write(differ_log, "").expect("Unable to write file");
    }
    let differ_log_contents = std::fs::read_to_string(differ_log).expect("Unable to read file");
    println!("Differ log contents: {}", differ_log_contents);

    let mut differ_log_contents = differ_log_contents.to_string();
    differ_log_contents.push_str(&format!("{}\n", file_loc));
    std::fs::write(differ_log, differ_log_contents).expect("Unable to write file");
}

fn diff() {
    todo!();
}

fn main() {
    let bincode_cfg = options();
    let args = DifferArgs::parse();
    match args.cmd {
        Some(Cmd::New(NewArgs {
            content,
            id,
            variant,
        })) => new(content, id, variant),
        Some(Cmd::Diff) => diff(),
        None => println!("No subcommand was used"),
    }
}
