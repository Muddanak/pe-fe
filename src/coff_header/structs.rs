use std::fmt::{Debug, Display, Formatter};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct CoffHeader {
    pub HE_MACHINEINFO: u16,
    pub HE_SECTIONS: u16,
    pub HE_DATESTAMP_UTC: u32,
    pub HE_DATESTAMP_LOC: u32,
    pub HE_POINTERTOSYMBOLS: u32,
    pub HE_NUMBEROFSYMBOLS: u32,
    pub HE_OPTIONAL: u16,
    pub HE_CHARACTERISTICS: u16,
    pub HE_MACHINEINFO_OFFSET: usize,
    pub HE_SECTIONS_OFFSET: usize,
    pub HE_DATESTAMP_OFFSET: usize,
    pub HE_POINTERTOSYMBOLS_OFFSET: usize,
    pub HE_NUMBEROFSYMBOLS_OFFSET: usize,
    pub HE_OPTIONAL_OFFSET: usize,
    pub HE_CHARACTERISTICS_OFFSET: usize,
}

#[allow(dead_code)]
impl CoffHeader {
    pub fn new() -> Self {
       Default::default()
    }
}

impl Display for CoffHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name of coff_header\t\tInfo of Header\t\t\t\tOffset\n\
            Machine Info:\t\t{}\t\t\t\t{}\n\
            Sections:\t\t{}\t\t\t\t\t{}\n\
            DateStamp:\t\t{}\t\t{}\n\
            DateStamp:\t\t{}\t{}\n\
            Pointer to Symbols:\t{}\t\t\t\t\t{}\n\
            Number Symbols:\t\t{}\t\t\t\t\t{}\n\
            Optional:\t\t{}\t\t\t\t\t{}\n\
            Characteristics:\t{}\t{}",
            self.HE_MACHINEINFO,
            self.HE_MACHINEINFO_OFFSET,
            self.HE_SECTIONS,
            self.HE_SECTIONS_OFFSET,
            self.HE_DATESTAMP_UTC,
            self.HE_DATESTAMP_OFFSET,
            self.HE_DATESTAMP_LOC,
            self.HE_DATESTAMP_OFFSET,
            self.HE_POINTERTOSYMBOLS,
            self.HE_POINTERTOSYMBOLS_OFFSET,
            self.HE_NUMBEROFSYMBOLS,
            self.HE_NUMBEROFSYMBOLS_OFFSET,
            self.HE_OPTIONAL,
            self.HE_OPTIONAL_OFFSET,
            self.HE_CHARACTERISTICS,
            self.HE_CHARACTERISTICS_OFFSET,
        )
    }
}
