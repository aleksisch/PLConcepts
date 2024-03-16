mod vm;
mod registry;
mod program;
mod instruction;

use clap::Parser;
use crate::vm::VM;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long, default_value_t = 0)]
    entrypoint: i32,
}

fn main() {
    let args = Args::parse();
    let mut vm = VM::new(args.input_file, args.entrypoint, 0);
    while vm.next_inst() {}
}