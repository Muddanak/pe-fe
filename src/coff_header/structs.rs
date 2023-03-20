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
        write!(f, "Machine: \t\t{:#04x}\nSections: \t\t{:#04x}\nDateStamp: \t\t{:#04x}\nSymbolsAt: \t\t{:#04x}\n# of Symbols: \t\t{:#04x}\nOptional: \t\t{:#04x}\nCharacteristics: \t{:#04x}\n\nDetails: \n{}",
        self.HE_MACHINEINFO, self.HE_SECTIONS, self.HE_DATESTAMP_UTC, self.HE_POINTERTOSYMBOLS, self.HE_NUMBEROFSYMBOLS, self.HE_OPTIONAL, self.HE_CHARACTERISTICS, self.HE_DETAILS)
    }
}

impl Display for CoffHeaderDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Machine: \t\t{}\nDatestamp UTC: \t\t{}\nCharacteristics: \t{}",
        self.MACHINE, self.DATESTAMP_UTC, self.CHARACTERISTICS)
    }
}