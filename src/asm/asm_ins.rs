
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum OpcodeIns {
    Add,
    And,
    Br,
    JmpRet,
    Jsr,
    Ld,
    Ldi,
    Ldr,
    Lea,
    Not,
    Rti,
    St,
    Sti,
    Str,
    Trap,
    Reserved,
    INVALID
}

#[allow(dead_code)]
impl OpcodeIns {
    pub fn from(opcode: u8) -> Self {
        match opcode {
            0 => Self::Br,
            1 => Self::Add,
            2 => Self::Ld,
            3 => Self::St,
            4 => Self::Jsr,
            5 => Self::And,
            6 => Self::Ldr,
            7 => Self::Str,
            8 => Self::Rti,
            9 => Self::Not,
            10 => Self::Ldi,
            11 => Self::Sti,
            12 => Self::JmpRet,
            13 => Self::Reserved,
            14 => Self::Lea,
            15 => Self::Trap,
            _ => Self::INVALID,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::asm::asm_ins::*;
 
    #[test]
    fn test_ins_map() {
        assert_eq!(OpcodeIns::from(0), OpcodeIns::Br);
        assert_eq!(OpcodeIns::from(1), OpcodeIns::Add);
        assert_eq!(OpcodeIns::from(2), OpcodeIns::Ld);
        assert_eq!(OpcodeIns::from(3), OpcodeIns::St);
        assert_eq!(OpcodeIns::from(4), OpcodeIns::Jsr);
        assert_eq!(OpcodeIns::from(5), OpcodeIns::And);
        assert_eq!(OpcodeIns::from(6), OpcodeIns::Ldr);
        assert_eq!(OpcodeIns::from(7), OpcodeIns::Str);
        assert_eq!(OpcodeIns::from(8), OpcodeIns::Rti);
        assert_eq!(OpcodeIns::from(9), OpcodeIns::Not);
        assert_eq!(OpcodeIns::from(10), OpcodeIns::Ldi);
        assert_eq!(OpcodeIns::from(11), OpcodeIns::Sti);
        assert_eq!(OpcodeIns::from(12), OpcodeIns::JmpRet);
        assert_eq!(OpcodeIns::from(13), OpcodeIns::Reserved);
        assert_eq!(OpcodeIns::from(14), OpcodeIns::Lea);
        assert_eq!(OpcodeIns::from(15), OpcodeIns::Trap);
    }

    #[test]
    fn test_ins_map_invalid() {
        assert_eq!(OpcodeIns::from(16), OpcodeIns::INVALID);

    }
}
