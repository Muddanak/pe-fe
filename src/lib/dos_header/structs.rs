use std::fmt::{Display, Formatter};
use druid::Data;

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Data)]
pub struct DosHeader {
    pub mz_offset: usize,
    pub pe_offset: usize,
    pub has_stub: bool,
    pub has_rich: bool,
    pub rich_xor_key: u32,

    #[data(eq)]
    pub rich_ids: Vec<u32>,
}

#[allow(unused_must_use)]
impl Display for DosHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "--------DOS Header--------\n\
        MZ:\t\t{:#x}\nPE:\t\t{:#x}\nStub:\t\t{}\n",
            self.mz_offset, self.pe_offset, self.has_stub
        );
        if self.has_rich {
            write!(
                f,
                "\n--------Rich Header--------\n\
            Has Rich:\t{}\t \nRich XOR Key:\t{:#X}\n",
                self.has_rich, self.rich_xor_key
            );
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl DosHeader {
    pub fn new() -> Self {
        Default::default()
    }
}
