use clap::Parser;
use von_neim::assembler::Assembly;
use von_neim::disassembler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let asm = disassembler::disassembly(args.input_file);
}