use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub struct DosHeader {
    pub mz_offset: usize,
    pub ox3c_offset: usize,
}

impl Display for DosHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOS Header\n---------\n--MZ\n    \\_{:#04X}\n--PE\n    \\_{:#04X}", self.mz_offset, self.ox3c_offset)
    }
}

#[allow(dead_code)]
impl DosHeader {
    pub fn new() -> Self {
        Self {
            mz_offset: 0,
            ox3c_offset: 0,
        }
    }
}