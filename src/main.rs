pub mod vm;
pub mod asm;

use crate::vm::vm::VM;
use crate::asm::asm::Asm;

fn main() {
    let vm = VM::new();
    let asm = Asm::new();
    println!("Hello, world!");
}
