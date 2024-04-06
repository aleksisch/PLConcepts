use clap::Parser;
use von_neim::assembler::Assembly;

/// Translator from assembly to binary
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Assembly file
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
