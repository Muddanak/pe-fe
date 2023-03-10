#[allow(dead_code)]
pub struct DosHeader {
    pub mz_offset: u32,
    pub ox3c_offset: u32,
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