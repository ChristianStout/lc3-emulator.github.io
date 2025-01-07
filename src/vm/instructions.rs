
/*
Uses the command pattern to execute functions dynamically
*/

pub trait Instruction {
    fn exe(&self, value: u16);
}

struct Add;
struct And;
struct Br;
struct JmpRet;
struct Jsr;
struct Ld;
struct Ldi;
struct Lea;
struct Not;
struct Rti;
struct St;
struct Sti;
struct Str;
struct Trap;

impl Instruction for Add {
    fn exe(&self, value: u16) {

    }
}
