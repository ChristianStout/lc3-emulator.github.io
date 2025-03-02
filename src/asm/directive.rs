
#[derive(Debug, Clone, PartialEq)]
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
}
