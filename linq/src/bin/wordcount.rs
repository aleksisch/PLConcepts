use std::fs::File;
use std::io;
use std::io::BufRead;
use clap::Parser;
use linq::linq_impl::LinqExt;

/// Translator from assembly to binary
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text file
    #[arg(short, long)]
    input_file: String,
}


fn main() {
    let args = Args::parse();

    let res: Vec<(String, usize)> = io::BufReader::new(File::open(args.input_file).unwrap())
        .lines()
        .flatten()
        .select(|line| line.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
        .flatten()
        .group_by(|l| l.clone())
        .select(|(k, v)| (k, v.len()))
        .order_by(|(_, v1), (_, v2)| v2.cmp(v1))
        .collect();
    println!("{:?}", res);
}