use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use super::asm_ins::OperandType;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Directive {
    // ORIG(u16),
    // FILL(u16),
    // BLKW(i16),
    // STRINGZ(String),
    ORIG,
    FILL,
    BLKW,
    STRINGZ,
    END,
}

impl Directive {
    pub fn from(word: &str) -> Directive {
        match word {
            ".ORIG" => return Directive::ORIG,
            ".FILL" => return Directive::FILL,
            ".BLKW" => return Directive::BLKW,
            ".STRINGZ" => return Directive::STRINGZ,
            ".END" => return Directive::END,
            _ => unreachable!(),
        }
    }

    pub fn get_expected_operands(&self) -> VecDeque<OperandType> {
        match self {
            Directive::ORIG | Directive::FILL | Directive::BLKW => vec![OperandType::Imm].into_iter().collect(),
            Directive::STRINGZ => vec![OperandType::String].into_iter().collect(),
            _ => vec![].into_iter().collect(),
        }
    }
}
