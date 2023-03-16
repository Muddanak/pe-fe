use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DosHeader {
    pub mz_offset: usize,
    pub ox3c_offset: usize,
    pub has_stub: bool,
    pub has_rich: bool,
    pub rich_xor_key: u32,
    pub rich_ids: Vec<u32>,
}

/*#[derive(Debug)]
pub struct RichHeader {
    pub rich_id: u64
}*/

impl Display for DosHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOS Header\n---------\n\
        --MZ        --DOS Stub        --Rich        --Rich XOR\n    \\_{:#04X}           \\_{}         \\_{}            \\_{:#04X}\n\
        --PE\n    \\_{:#04X}", self.mz_offset, self.has_stub, self.has_rich, self.rich_xor_key, self.ox3c_offset)
    }
}

#[allow(dead_code)]
impl DosHeader {
    pub fn new() -> Self {
        Default::default()
    }
}