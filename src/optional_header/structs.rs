use std::fmt::{Display, Formatter};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct OptHeader {
    pub MAGIC: u16,
    pub MAJORLINKER: u8,
    pub MINORLINKER: u8,
    pub SIZEOFCODE: u32,
    pub SIZEOFINITDATA: u32,
    pub SIZEOFUNINITDATA: u32,
    pub ADDROFENTRYPOINT: u32,
    pub BASEOFCODE: u32,
    pub DETAILS: OptHeaderDetails,
}

#[allow(dead_code)]
impl OptHeader {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Optional Header Information\n\
        --Magic  --MajorLinker  --MinorLinker  --Size of Code\
        \n       \\_{:#04X}        \\_{}           \\_{}            \\_{}\n\
        --Size Init Data  --Size Uninit Data  --Addr Entry Point --Base of Code\
        \n                \\_{}           \\_{}                  \\_{:#04X}         \\_{:#04X}\
        \nDetails:\n{}",
        self.MAGIC,
        self.MAJORLINKER,
        self.MINORLINKER,
        self.SIZEOFCODE,
        self.SIZEOFINITDATA,
        self.SIZEOFUNINITDATA,
        self.ADDROFENTRYPOINT,
        self.BASEOFCODE,

        self.DETAILS)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct OptHeaderDetails {
    pub MAGIC: String,
}

#[allow(dead_code)]
impl OptHeaderDetails {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeaderDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Magic: \t{}",
        self.MAGIC,)
    }
}

