use std::fmt::{Display, Formatter};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Header {
    pub HE_MACHINEINFO: String,
    pub HE_SECTIONS: String,
    pub HE_DATESTAMP_UTC: String,
    pub HE_DATESTAMP_LOC: String,
    pub HE_POINTERTOSYMBOLS: String,
    pub HE_NUMBEROFSYMBOLS: String,
    pub HE_OPTIONAL: String,
    pub HE_CHARACTERISTICS: String,
    pub HE_MACHINEINFO_OFFSET: String,
    pub HE_SECTIONS_OFFSET: String,
    pub HE_DATESTAMP_OFFSET: String,
    pub HE_POINTERTOSYMBOLS_OFFSET: String,
    pub HE_NUMBEROFSYMBOLS_OFFSET: String,
    pub HE_OPTIONAL_OFFSET: String,
    pub HE_CHARACTERISTICS_OFFSET: String,
}

impl Header {
    #[allow(clippy::too_many_arguments)]
    pub fn _create(
        machine: String,
        sections: String,
        datestamp: String,
        datestamploc: String,
        p2symbols: String,
        numsymbols: String,
        optional: String,
        characteristics: String,

        machine_offset: String,
        sections_offset: String,
        datestamp_offset: String,
        p2symbols_offset: String,
        numsymbols_offset: String,
        optional_offset: String,
        characteristics_offset: String,
    ) -> Self {
        Self {
            HE_MACHINEINFO: machine,
            HE_SECTIONS: sections,
            HE_DATESTAMP_UTC: datestamp,
            HE_DATESTAMP_LOC: datestamploc,
            HE_POINTERTOSYMBOLS: p2symbols,
            HE_NUMBEROFSYMBOLS: numsymbols,
            HE_OPTIONAL: optional,
            HE_CHARACTERISTICS: characteristics,
            HE_MACHINEINFO_OFFSET: machine_offset,
            HE_SECTIONS_OFFSET: sections_offset,
            HE_DATESTAMP_OFFSET: datestamp_offset,
            HE_POINTERTOSYMBOLS_OFFSET: p2symbols_offset,
            HE_NUMBEROFSYMBOLS_OFFSET: numsymbols_offset,
            HE_OPTIONAL_OFFSET: optional_offset,
            HE_CHARACTERISTICS_OFFSET: characteristics_offset,
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            HE_MACHINEINFO: "".to_string(),
            HE_SECTIONS: "".to_string(),
            HE_DATESTAMP_UTC: "".to_string(),
            HE_DATESTAMP_LOC: "".to_string(),
            HE_POINTERTOSYMBOLS: "".to_string(),
            HE_NUMBEROFSYMBOLS: "".to_string(),
            HE_OPTIONAL: "".to_string(),
            HE_CHARACTERISTICS: "".to_string(),
            HE_MACHINEINFO_OFFSET: "".to_string(),
            HE_SECTIONS_OFFSET: "".to_string(),
            HE_DATESTAMP_OFFSET: "".to_string(),
            HE_POINTERTOSYMBOLS_OFFSET: "".to_string(),
            HE_NUMBEROFSYMBOLS_OFFSET: "".to_string(),
            HE_OPTIONAL_OFFSET: "".to_string(),
            HE_CHARACTERISTICS_OFFSET: "".to_string(),
        }
    }
}

impl Display for Header {
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
