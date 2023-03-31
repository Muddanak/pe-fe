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
        write!(
            f,
            "\n------Optional Header Information--------\n\
            Magic:\t\t{:#x}\t\nMajor Linker:\t{:#}\t\nMinor Linker:\t{:#}\n\
            Code Size:\t{:#}\nInit Size:\t{:#}\nUninit Size:\t{:#}\n\
            Entry Point:\t{:#x}\nBase of Code:\t{:#x}\nBase of Data:\t{:#x}\n\n\
            --------Details--------\n{}",
            self.MAGIC,
            self.MAJORLINKER,
            self.MINORLINKER,
            self.SIZEOFCODE,
            self.SIZEOFINITDATA,
            self.SIZEOFUNINITDATA,
            self.ADDROFENTRYPOINT,
            self.BASEOFCODE,
            self.BASEOFDATA,
            self.DETAILS,
        )
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
        write!(f, "Magic: \t\t\t{}", self.MAGIC,)
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
        write!(
            f,
            "Image Base:\t\t{:#x}\nWin32 Version:\t\t{:#}\n\
               Section Alignment:\t{:#x}\nFile Alignment:\t\t{:#x}\n\
               Major OS Version:\t{:#}\nMinor OS Version:\t{:#}\n\
               Major IMG Version:\t{:#}\nMinor IMG Version:\t{:#}\n\
               Major Subsys Ver:\t{:#}\nMinor Subsys Ver:\t{:#}\n\
               Size of IMG:\t\t{:#}\nSize of Headers:\t{:#}\n\
               Checksum:\t\t{:#x}\nSubsystem:\t\t{}\t\n\
               Size of Stack Reserve:\t{:#x}\nSize of Stack Commit:\t{:#x}\n\
               Size of Heap Reserve: \t{:#x}\nSize of Heap Commit:\t{:#x}\n\
               Loader Flags:\t\t{:#x}\n# of RVA and Sizes:\t{}\n\nDLL Characteristics: {}",
            self.IMAGEBASE,
            self.WIN32VERSION,
            self.SECTIONALIGNMENT,
            self.FILEALIGNMENT,
            self.MAJOROSVERSION,
            self.MINOROSVERSION,
            self.MAJORIMGVERSION,
            self.MINORIMGVERSION,
            self.MAJORSUBVERSION,
            self.MINORSUBVERSION,
            self.SIZEOFIMAGE,
            self.SIZEOFHEADERS,
            self.CHECKSUM,
            self.SUBSYSTEM,
            self.SIZEOFSTACKRESERVE,
            self.SIZEOFSTACKCOMMIT,
            self.SIZEOFHEAPRESERVE,
            self.SIZEOFHEAPCOMMIT,
            self.LOADERFLAGS,
            self.NUMBERRVASIDES,
            self.DLLCHARACTERISTICS,
        )
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
        write!(
            f,
            "Image Base: {:#x} |\tSection Alignment: {:#x} |\tFile Alignment: {:#x}\n\
               Major OS Version: {:#x} |\tMinor OS Version: {:#x} |\tMajor IMG Version: {:#x}\n\
               Minor IMG Version: {:#x} |\tMajor Subsys Ver: {:#x} |\t Minor Subsys Ver: {:#x}\n\
               Win32 Version: {:#x} |\tSize of IMG: {:#x} |\tSize of Headers: {:#x}\n\
               Checksum: {:#x} |\tSubsystem: {} |\t\nDLL Characteristics: {}\n\
               Size of Stack Reserve: {:#x} |\tSize of Stack Commit: {:#x}\n\
               Size of Heap Reserve: {:#x} |\tSize of Heap Commit: {:#x}\n\
               Loader Flags: {:#x} |\tNumber of RVA and Sizes: {:#x}",
            self.IMAGEBASE,
            self.SECTIONALIGNMENT,
            self.FILEALIGNMENT,
            self.MAJOROSVERSION,
            self.MINOROSVERSION,
            self.MAJORIMGVERSION,
            self.MINORIMGVERSION,
            self.MAJORSUBVERSION,
            self.MINORSUBVERSION,
            self.WIN32VERSION,
            self.SIZEOFIMAGE,
            self.SIZEOFHEADERS,
            self.CHECKSUM,
            self.SUBSYSTEM,
            self.DLLCHARACTERISTICS,
            self.SIZEOFSTACKRESERVE,
            self.SIZEOFSTACKCOMMIT,
            self.SIZEOFHEAPRESERVE,
            self.SIZEOFHEAPCOMMIT,
            self.LOADERFLAGS,
            self.NUMBERRVASIDES
        )
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct OptHeaderDataDirectories {
    pub EXPORTTABLE: (u32, u32),
    pub IMPORTTABLE: (u32, u32),
    pub RESOURCETABLE: (u32, u32),
    pub EXCEPTIONTABLE: (u32, u32),
    pub CERTTABLE: (u32, u32),
    pub BASERELOCATION: (u32, u32),
    pub DEBUG: (u32, u32),
    pub ARCHITECTURE: (u32, u32),
    pub GLOBALPTR: (u32, u32),
    pub TLSTABLE: (u32, u32),
    pub LOADCONFIG: (u32, u32),
    pub BOUNDIMPORT: (u32, u32),
    pub IAT: (u32, u32),
    pub DELAYIMPDESC: (u32, u32),
    pub CLRRUNTIME: (u32, u32),
    pub RESERVEDZERO: (u32, u32),
}

#[allow(dead_code)]
impl OptHeaderDataDirectories {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for OptHeaderDataDirectories {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n--------Data Directories--------\n\
        Export:\t\t{:#08x}| Size: {:#}\n\
        Import:\t\t{:#08x}| Size: {:#}\n\
        Resource:\t{:#08x}| Size: {:#}\n\
        Exception:\t{:#08x}| Size: {:#}\n\
        Certificate:\t{:#08x}| Size: {:#}\n\
        Base Reloc:\t{:#08x}| Size: {:#}\n\
        Debug:\t\t{:#08x}| Size: {:#}\n\
        Architecture:\t{:#08x}| Size: {:#}\n\
        Global Ptr:\t{:#08x}| Size: {:#}\n\
        TLS:\t\t{:#08x}| Size: {:#}\n\
        Load Config:\t{:#08x}| Size: {:#}\n\
        Bound Import:\t{:#08x}| Size: {:#}\n\
        IAT:\t\t{:#08x}| Size: {:#}\n\
        Delay Import:\t{:#08x}| Size: {:#}\n\
        CLR Runtime:\t{:#08x}| Size: {:#}\n\
        ReservedZero:\t{:#08x}| Size: {:#}\n",
            self.EXPORTTABLE.1, self.EXPORTTABLE.0,
            self.IMPORTTABLE.1, self.IMPORTTABLE.0,
            self.RESOURCETABLE.1, self.RESOURCETABLE.0,
            self.EXCEPTIONTABLE.1, self.EXCEPTIONTABLE.0,
            self.CERTTABLE.1, self.CERTTABLE.0,
            self.BASERELOCATION.1, self.BASERELOCATION.0,
            self.DEBUG.1, self.DEBUG.0,
            self.ARCHITECTURE.1, self.ARCHITECTURE.0,
            self.GLOBALPTR.1, self.GLOBALPTR.0,
            self.TLSTABLE.1, self.TLSTABLE.0,
            self.LOADCONFIG.1, self.LOADCONFIG.0,
            self.BOUNDIMPORT.1, self.BOUNDIMPORT.0,
            self.IAT.1, self.IAT.0,
            self.DELAYIMPDESC.1, self.DELAYIMPDESC.0,
            self.CLRRUNTIME.1, self.CLRRUNTIME.0,
            self.RESERVEDZERO.1, self.RESERVEDZERO.0,
        )
    }
}
