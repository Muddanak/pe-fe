use std::fmt::{Display, Formatter};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct SectionHeader {
    pub HEADER: Vec<SectionHeaderInfo>,
}

#[allow(dead_code)]
impl SectionHeader {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for SectionHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------Section Headers--------\n")?;
        for (_count, item) in self.HEADER.iter().enumerate() {
            write!(f, "{}", item)?
        }
        writeln!(f)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct SectionHeaderInfo {
    pub NAME: u64,
    pub VIRTUALSIZE: u32,
    pub VIRTUALADDRESS: u32,
    pub SIZEOFRAWDATA: u32,
    pub POINTERTORAWDATA: u32,
    pub POINTERTORELOCATIONS: u32,
    pub POINTERTOLINENUMBERS: u32,
    pub NUMBEROFRELOCATIONS: u32,
    pub NUMBEROFLINENUMBERS: u32,
    pub CHARACTERISTICS: String,
}

#[allow(dead_code)]
impl SectionHeaderInfo {
    fn new() -> Self {
        Default::default()
    }
}

impl Display for SectionHeaderInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name:\t{}\n\
        Virtual Size:\t{}\n\
        Virtual Address:\t{}\
        Size of Raw Data:\t{}\
        Pointer to Raw:\t{}\
        Pointer to Reloc:\t{}\
        Pointer to Line Num:\t{}\
        Number of Relocations:\t{}\
        Number of Lines:\t{}\
        Characteristics:\t{}\n",
        self.NAME,
        self.VIRTUALSIZE,
        self.VIRTUALADDRESS,
        self.SIZEOFRAWDATA,
        self.POINTERTORAWDATA,
        self.POINTERTORELOCATIONS,
        self.POINTERTOLINENUMBERS,
        self.NUMBEROFRELOCATIONS,
        self.NUMBEROFLINENUMBERS,
        self.CHARACTERISTICS)
    }
}