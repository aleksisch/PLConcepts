use clap::Parser;
use von_neim::disassembler;

/// Disassembler binary to asm
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Binary file
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    disassembler::disassembly(args.input_file);
}
