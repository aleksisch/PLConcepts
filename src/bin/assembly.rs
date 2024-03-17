use clap::Parser;
use von_neim::assembler::Assembly;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let asm = Assembly::new();
    asm.parse(args.input_file, args.output_file)
}