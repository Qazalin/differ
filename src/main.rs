use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct DifferArgs {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Args, Debug)]
struct DiffArgs {
    /// files to compare
    #[clap(num_args(2))]
    file: Vec<String>,
    /// only diff lines that contain this
    #[clap(last = true)]
    pattern: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Diff(DiffArgs),
}

fn main() {
    let args = DifferArgs::parse();
    match args.cmd {
        Some(Cmd::Diff(args)) => {
            let file_contents = args
                .file
                .iter()
                .map(|file| std::fs::read_to_string(file).unwrap())
                .collect::<Vec<String>>();
            let (v1, v2) = (&file_contents[0], &file_contents[1]);
            let diffable = usize::min(v1.len(), v2.len());
            let good_lines = Vec::from_iter(v1.lines());
            for (i, compare) in v2.lines().enumerate() {
                let good = good_lines[i];
                let diff = match &args.pattern {
                    Some(pat) => good.contains(pat) && compare != good,
                    None => compare != good,
                };
                match diff {
                    false => {
                        println!("{good}");
                    }
                    true => {
                        println!("\x1b[31m- {}\x1b[0m", good);
                        println!("\x1b[32m+ {}\x1b[0m", compare);
                    }
                }
            }
            if diffable < usize::max(v1.len(), v2.len()) {
                println!("\x1b[33mdiffed less lines\x1b[0m");
            }
        }
        None => println!("No subcommand was used"),
    }
}
