use clap::Parser;

/// Expect file has moved or not and the file path
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Passed in from caller whether file was moved.
    #[clap(short, long, action)]
    has_moved: bool,
    /// Path of file whether moved or not.
    #[clap(short, long)]
    file_path: String,
}


fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // println!("Hello, world! {} {}", pattern, path);

    let args = Cli::parse();
    println!("{:?} {:?}", args.has_moved, args.file_path);
}
