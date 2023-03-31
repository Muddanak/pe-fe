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
    pub NUMBEROFRELOCATIONS: u16,
    pub NUMBEROFLINENUMBERS: u16,
    pub CHARACTERISTICS: String,
}

#[allow(dead_code)]
impl SectionHeaderInfo {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Display for SectionHeaderInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name:\t\t\t{}\n\
        Virtual Size:\t\t{}\n\
        Virtual Address:\t{:#x}\n\
        Size of Raw Data:\t{}\n\
        Pointer to Raw:\t\t{:#x}\n\
        Pointer to Reloc:\t{:#x}\n\
        Pointer to Line Num:\t{:#x}\n\
        Number of Relocations:\t{}\n\
        Number of Lines:\t{}\n\
        Characteristics:\t{}\n\n",
            String::from_utf8(Vec::from(u64::to_le_bytes(self.NAME))).unwrap(),
            self.VIRTUALSIZE,
            self.VIRTUALADDRESS,
            self.SIZEOFRAWDATA,
            self.POINTERTORAWDATA,
            self.POINTERTORELOCATIONS,
            self.POINTERTOLINENUMBERS,
            self.NUMBEROFRELOCATIONS,
            self.NUMBEROFLINENUMBERS,
            self.CHARACTERISTICS
        )
    }
}
