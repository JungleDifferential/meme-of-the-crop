use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "crops all images in the specified path")]
struct Args {
    #[arg(short, long)]
    path: Option<PathBuf>,
    
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // get path. default to "."
    let path = args.path.unwrap_or_else(|| ".".into());
    let verbose = args.verbose;

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        println!("{}", p.display());
    }
}
