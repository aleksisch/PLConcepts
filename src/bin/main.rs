use clap::Parser;
use von_neim::vm::VM;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let mut vm = VM::new(args.input_file);
    while (vm.next_inst()) {}
}