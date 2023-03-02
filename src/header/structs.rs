use std::fmt::{Display, Formatter};

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Clone)]
pub struct Header {
    pub HE_MACHINEINFO: String,
    pub HE_SECTIONS: String,
    pub HE_DATESTAMP: String,
    pub HE_P2SYMBOLS: String,
    pub HE_NUMSYMBOLS: String,
    pub HE_OPTIONAL: String,
    pub HE_CHARACTERISTICS:String,
}

impl Header {
    pub fn new(machine: String, sections: String, datestamp: String, p2symbols: String, numsymbols: String, optional: String, characteristics: String) -> Self {
        Self {
            HE_MACHINEINFO: machine,
            HE_SECTIONS: sections,
            HE_DATESTAMP: datestamp,
            HE_P2SYMBOLS: p2symbols,
            HE_NUMSYMBOLS: numsymbols,
            HE_OPTIONAL: optional,
            HE_CHARACTERISTICS: characteristics,
        }
    }
}

impl Display for Header{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Machine Info:\t\t{}\n\
                Sections:\t\t{}\n\
                DateStamp:\t\t{}\n\
                P2 Symbols:\t\t{}\n\
                Number Symbols:\t\t{}\n\
                Optional:\t\t{}\n\
                Characteristics:\t{}",
               self.HE_MACHINEINFO, self.HE_SECTIONS, self.HE_DATESTAMP, self.HE_P2SYMBOLS, self.HE_NUMSYMBOLS, self.HE_OPTIONAL, self.HE_CHARACTERISTICS)
    }
}
