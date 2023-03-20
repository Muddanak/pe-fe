use phf::phf_map;

#[allow(dead_code)]
pub static MAGIC: phf::Map<&'static str, u16> = phf_map!(
    "PE32" => 0x10b,
    "PE32+" => 0x20b,
);

