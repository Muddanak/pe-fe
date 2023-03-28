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
    pub WINDETAILSPLUS: OptHeaderPE32PlusDetails,
    pub WINDETAILS32: OptHeaderPE32Details,
    pub DATADIRECTORIES: OptHeaderDataDirectories,
}

#[allow(dead_code)]
impl OptHeader {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n------Optional Header Information--------\n\
        --Magic  --MajorLinker  --MinorLinker  --Size of Code\
        \n       \\_{:#x}        \\_{}           \\_{}            \\_{}\n\
        --Size Init Data  --Size Uninit Data  --Entry Point --Base of Code  --Base of Data\
        \n           \\_{}           \\_{}            \\_{:#x}         \\_{:#x}      \\_{:#x}\
        \n\n--------Details--------\n{}",
        self.MAGIC,
        self.MAJORLINKER,
        self.MINORLINKER,
        self.SIZEOFCODE,
        self.SIZEOFINITDATA,
        self.SIZEOFUNINITDATA,
        self.ADDROFENTRYPOINT,
        self.BASEOFCODE,
        self.BASEOFDATA,

        self.DETAILS,)
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
pub struct OptHeaderPE32PlusDetails {
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
impl OptHeaderPE32PlusDetails {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeaderPE32PlusDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Image Base: {:#x} |\tSection Alignment: {:#x} |\tFile Alignment: {:#x}\n\
               Major OS Version: {:#x} |\tMinor OS Version: {:#x} |\tMajor IMG Version: {:#x}\n\
               Minor IMG Version: {:#x} |\tMajor Subsys Ver: {:#x} |\t Minor Subsys Ver: {:#x}\n\
               Win32 Version: {:#x} |\tSize of IMG: {:#x} |\tSize of Headers: {:#x}\n\
               Checksum: {:#x} |\tSubsystem: {} |\t\nDLL Characteristics: {}\n\
               Size of Stack Reserve: {:#x} |\tSize of Stack Commit: {:#x}\n\
               Size of Heap Reserve: {:#x} |\tSize of Heap Commit: {:#x}\n\
               Loader Flags: {:#x} |\tNumber of RVA and Sizes: {}",
               self.IMAGEBASE, self.SECTIONALIGNMENT, self.FILEALIGNMENT, self.MAJOROSVERSION, self.MINOROSVERSION, self.MAJORIMGVERSION, self.MINORIMGVERSION,
        self.MAJORSUBVERSION, self.MINORSUBVERSION, self.WIN32VERSION, self.SIZEOFIMAGE, self.SIZEOFHEADERS, self.CHECKSUM, self.SUBSYSTEM, self.DLLCHARACTERISTICS,
        self.SIZEOFSTACKRESERVE, self.SIZEOFSTACKCOMMIT, self.SIZEOFHEAPRESERVE, self.SIZEOFHEAPCOMMIT, self.LOADERFLAGS, self.NUMBERRVASIDES)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct OptHeaderPE32Details {
    pub IMAGEBASE: u32,
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
    pub SIZEOFSTACKRESERVE: u32,
    pub SIZEOFSTACKCOMMIT: u32,
    pub SIZEOFHEAPRESERVE: u32,
    pub SIZEOFHEAPCOMMIT: u32,
    pub LOADERFLAGS: u32,
    pub NUMBERRVASIDES: u32,
}

#[allow(dead_code)]
impl OptHeaderPE32Details {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeaderPE32Details {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Image Base: {:#x} |\tSection Alignment: {:#x} |\tFile Alignment: {:#x}\n\
               Major OS Version: {:#x} |\tMinor OS Version: {:#x} |\tMajor IMG Version: {:#x}\n\
               Minor IMG Version: {:#x} |\tMajor Subsys Ver: {:#x} |\t Minor Subsys Ver: {:#x}\n\
               Win32 Version: {:#x} |\tSize of IMG: {:#x} |\tSize of Headers: {:#x}\n\
               Checksum: {:#x} |\tSubsystem: {} |\t\nDLL Characteristics: {}\n\
               Size of Stack Reserve: {:#x} |\tSize of Stack Commit: {:#x}\n\
               Size of Heap Reserve: {:#x} |\tSize of Heap Commit: {:#x}\n\
               Loader Flags: {:#x} |\tNumber of RVA and Sizes: {:#x}",
               self.IMAGEBASE, self.SECTIONALIGNMENT, self.FILEALIGNMENT, self.MAJOROSVERSION, self.MINOROSVERSION, self.MAJORIMGVERSION, self.MINORIMGVERSION,
               self.MAJORSUBVERSION, self.MINORSUBVERSION, self.WIN32VERSION, self.SIZEOFIMAGE, self.SIZEOFHEADERS, self.CHECKSUM, self.SUBSYSTEM, self.DLLCHARACTERISTICS,
               self.SIZEOFSTACKRESERVE, self.SIZEOFSTACKCOMMIT, self.SIZEOFHEAPRESERVE, self.SIZEOFHEAPCOMMIT, self.LOADERFLAGS, self.NUMBERRVASIDES)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct OptHeaderDataDirectories {
    pub EXPORTTABLE: u64,
    pub IMPORTTABLE: u64,
    pub RESOURCETABLE: u64,
    pub EXCEPTIONTABLE: u64,
    pub CERTTABLE: u64,
    pub BASERELOCATION: u64,
    pub DEBUG: u64,
    pub ARCHITECTURE: u64,
    pub GLOBALPTR: u64,
    pub TLSTABLE: u64,
    pub LOADCONFIG: u64,
    pub BOUNDIMPORT: u64,
    pub IAT: u64,
    pub DELAYIMPDESC: u64,
    pub CLRRUNTIME: u64,
    pub RESERVEDZERO: u64
}

#[allow(dead_code)]
impl OptHeaderDataDirectories {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeaderDataDirectories {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------Data Directories--------\n\
        Export:\t\t{:#x}\t\tImport:\t{:#x}\tResource:\t{:#x}\n\
        Exception:\t{:#x}\tCertificate:\t{:#x}\tBase Relocation:\t{:#x}\n\
        Debug:\t{:#x}\tArchitecture:\t{:#x}\tGlobal Ptr:\t{:#x}\t\n\
        TLS:\t{:#x}\tLoad Config:\t{:#x}\tBound Import:\t{:#x}\t\n\
        IAT:\t{:#x}\tDelay Import Descriptor:\t{:#x}\t\n\
        CLR Runtime Header:\t{:#x}\tReservedZero:\t{:#x}\t",
               self.EXPORTTABLE, self.IMPORTTABLE, self.RESOURCETABLE, self.EXCEPTIONTABLE,
        self.CERTTABLE, self.BASERELOCATION, self.DEBUG, self.ARCHITECTURE, self.GLOBALPTR,
        self.TLSTABLE, self.LOADCONFIG, self.BOUNDIMPORT, self.IAT, self.DELAYIMPDESC,
        self.CLRRUNTIME, self.RESERVEDZERO)
    }
}