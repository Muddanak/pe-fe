use std::fmt::{Debug, Display, Formatter};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct CoffHeader {
    pub HE_MACHINEINFO: u16,
    pub HE_SECTIONS: u16,
    pub HE_DATESTAMP_UTC: u32,
    pub HE_POINTERTOSYMBOLS: u32,
    pub HE_NUMBEROFSYMBOLS: u32,
    pub HE_OPTIONAL: u16,
    pub HE_CHARACTERISTICS: u16,
    pub HE_DETAILS: CoffHeaderDetails,
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct CoffHeaderDetails {
    pub MACHINE: String,
    pub DATESTAMP_UTC: String,
    pub CHARACTERISTICS: String,
}

#[allow(dead_code)]
impl CoffHeader {
    pub fn new() -> Self {
        Default::default()
    }
}

#[allow(dead_code)]
impl CoffHeaderDetails {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Display for CoffHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------COFF Header--------\nMachine:\t{:#x}\t\t| Sections:\t{:#x}\t| DateStamp:\t{}\n\
        SymbolsAt:\t{:#x}\t| # of Symbols:\t{:#x}\nOptional: \t\t{:#x}\nCharacteristics: \t{:#b}\n\n{}",
        self.HE_MACHINEINFO, self.HE_SECTIONS, self.HE_DATESTAMP_UTC, self.HE_POINTERTOSYMBOLS, self.HE_NUMBEROFSYMBOLS, self.HE_OPTIONAL, self.HE_CHARACTERISTICS, self.HE_DETAILS)
    }
}

impl Display for CoffHeaderDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Machine: \t\t{}\nDatestamp UTC: \t\t{}\nCharacteristics: \t{}\n",
            self.MACHINE, self.DATESTAMP_UTC, self.CHARACTERISTICS
        )
    }
}
