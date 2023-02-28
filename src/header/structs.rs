#[allow(non_snake_case)]
pub struct Header {
    pub HE_MACHINEINFO: u16,
    pub HE_SECTIONS: u16,
    pub HE_DATESTAMP: u32,
    pub HE_P2SYMBOLS: u32,
    pub HE_NUMSYMBOLS: u32,
    pub HE_OPTIONAL: u16,
    pub HE_CHARACTERISTICS: u16,
}