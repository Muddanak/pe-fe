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
    pub BASEOFDATA: u32,
    pub DETAILS: OptHeaderDetails,
    pub WINDETAILS: OptHeaderWindowsDetails,
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
        \nDetails:\n{}\n{}",
        self.MAGIC,
        self.MAJORLINKER,
        self.MINORLINKER,
        self.SIZEOFCODE,
        self.SIZEOFINITDATA,
        self.SIZEOFUNINITDATA,
        self.ADDROFENTRYPOINT,
        self.BASEOFCODE,

        self.DETAILS,
        self.WINDETAILS)
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

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct OptHeaderWindowsDetails {
    pub IMAGEBASE: u64,
    pub SECTIONALIGNMENT: u32,
    pub FILEALIGNMENT: u32,
    pub MAJOROSVERSION: u16,
    pub MINOROSVERSION: u16,
    pub MAJORIMGVERSION: u16,
    pub MINORIMGVERSION: u16,
    pub MAJORSUBVERSION: u16,
    pub MINORSUBVERSION: u16,
    pub WIN32VERSION: u32,
    pub SIZEOFIMAGE: u32,
    pub SIZEOFHEADERS: u32,
    pub CHECKSUM: u32,
    pub SUBSYSTEM: String,
    pub DLLCHARACTERISTICS: String,
    pub SIZEOFSTACKRESERVE: u64,
    pub SIZEOFSTACKCOMMIT: u64,
    pub SIZEOFHEAPRESERVE: u64,
    pub SIZEOFHEAPCOMMIT: u64,
    pub LOADERFLAGS: u32,
    pub NUMBERRVASIDES: u32,
}

#[allow(dead_code)]
impl OptHeaderWindowsDetails {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeaderWindowsDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Image Base: {:#04x} |\tSection Alignment: {:#04x} |\tFile Alignment: {:#04x}\n\
               Major OS Version: {:#04x} |\tMinor OS Version: {:#04x} |\tMajor IMG Version: {:#04x}\n\
               Minor IMG Version: {:#04x} |\tMajor Subsys Ver: {:#04x} |\t Minor Subsys Ver: {:#04x}\n\
               Win32 Version: {:#04x} |\tSize of IMG: {:#04x} |\tSize of Headers: {:#04x}\n\
               Checksum: {:#04x} |\tSubsystem: {} |\t\nDLL Characteristics: {}\n\
               Size of Stack Reserve: {:#04x} |\tSize of Stack Commit: {:#04x}\n\
               Size of Heap Reserve: {:#04x} |\tSize of Heap Commit: {:#04x}\n\
               Loader Flags: {:#04x} |\tNumber of RVA and Sizes: {:#04x}",
               self.IMAGEBASE, self.SECTIONALIGNMENT, self.FILEALIGNMENT, self.MAJOROSVERSION, self.MINOROSVERSION, self.MAJORIMGVERSION, self.MINORIMGVERSION,
        self.MAJORSUBVERSION, self.MINORSUBVERSION, self.WIN32VERSION, self.SIZEOFIMAGE, self.SIZEOFHEADERS, self.CHECKSUM, self.SUBSYSTEM, self.DLLCHARACTERISTICS,
        self.SIZEOFSTACKRESERVE, self.SIZEOFSTACKCOMMIT, self.SIZEOFHEAPRESERVE, self.SIZEOFHEAPCOMMIT, self.LOADERFLAGS, self.NUMBERRVASIDES)
    }
}

