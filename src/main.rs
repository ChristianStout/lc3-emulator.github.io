pub mod vm;
pub mod asm;
pub mod web;
pub mod output;

use crate::vm::vm::VM;
use crate::asm::asm::Asm;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Assume argument 1 is the file path
/*     let file_path = args.get(1)
        .expect("Must provide a file path") */;

    let file_path = "SecretProject.asm";
    // let file_path = "test.asm";
    let file = fs::read_to_string(file_path)
        .expect("The provided file path was not valid");


    let mut asm = Asm::new();
    let binary_file = asm.run(file);
    
    let mut vm = VM::new();
    vm.run(binary_file);
}
