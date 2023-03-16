use std::ops::{BitAnd, Shr};

#[allow(dead_code)]
pub fn u32_to_u16_high_low(inp: u32) -> (u16, u16) {
    let high: u16 = inp.bitand(0xFFFF0000).shr(16) as u16;
    let low: u16 = inp.bitand(0x0000FFFF) as u16;
    (high, low)
}