use phf::phf_map;
use std::fmt::{Display, Formatter};

///
///
/// Errors
/// Current:
///
///
pub enum _FileErrors {
    CouldNotOpenFile(std::io::Error),
}

#[allow(dead_code, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum PEFILEERROR {
    CouldNotGetData,
    OffsetPEisZero,
    NoMZinFile,
    CouldNotGetOffset,
    PEisNotHere,
    PEInvalid,
}

impl std::error::Error for PEFILEERROR {}

impl Display for PEFILEERROR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PEFILEERROR::CouldNotGetData => {
                write!(f, "Could not successfully get data from the file")
            }
            PEFILEERROR::OffsetPEisZero => write!(
                f,
                "The value located at offset 0x3c was zero, file might not be a PE"
            ),
            PEFILEERROR::NoMZinFile => {
                write!(f, "The identifier 'MZ' was not located in the DOS header")
            }
            PEFILEERROR::CouldNotGetOffset => {
                write!(f, "Somehow the offset 0x3c was unable to be found")
            }
            PEFILEERROR::PEisNotHere => {
                write!(f, "The PE coff_header was not found at the provided offset")
            }
            PEFILEERROR::PEInvalid => write!(f, "The PE is invalid, somehow?"),
        }
    }
}

#[allow(dead_code)]
pub static MACHINE: phf::Map<&'static str, u16> = phf_map!(
    "MACHINE_UNKNOWN" =>        0x0,
    "MACHINE_ALPHA" => 	        0x184,
    "MACHINE_ALPHA64" =>        0x284,
    "MACHINE_AM33" => 	        0x1d3,
    "MACHINE_AMD64" => 	        0x8664,
    "MACHINE_ARM" => 	        0x1c0,
    "MACHINE_ARM64" => 	        0xaa64,
    "MACHINE_ARMNT" => 	        0x1c4,
    "MACHINE_AXP64" => 	        0x284,
    "MACHINE_EBC" => 	        0xebc,
    "MACHINE_I386" => 	        0x14c,
    "MACHINE_IA64" => 	        0x200,
    "MACHINE_LOONGARCH32" => 	0x6232,
    "MACHINE_LOONGARCH64" => 	0x6264,
    "MACHINE_M32R" => 	        0x9041,
    "MACHINE_MIPS16" => 	    0x266,
    "MACHINE_MIPSFPU" => 	    0x366,
    "MACHINE_MIPSFPU16" => 	    0x466,
    "MACHINE_POWERPC" => 	    0x1f0,
    "MACHINE_POWERPCFP" => 	    0x1f1,
    "MACHINE_R4000" => 	        0x166,
    "MACHINE_RISCV32" => 	    0x5032,
    "MACHINE_RISCV64" => 	    0x5064,
    "MACHINE_RISCV128" => 	    0x5128,
    "MACHINE_SH3" => 	        0x1a2,
    "MACHINE_SH3DSP" => 	    0x1a3,
    "MACHINE_SH4" => 	        0x1a6,
    "MACHINE_SH5" => 	        0x1a8,
    "MACHINE_THUMB" => 	        0x1c2,
    "MACHINE_WCEMIPSV2" => 	    0x169,
);

#[allow(dead_code)]
pub static CHARACTERISTICS: phf::Map<&'static str, u16> = phf_map!(
    "RELOCS_STRIPPED"   =>          0x0001,
    "EXECUTABLE_IMAGE"   =>         0x0002,
    "LINE_NUMS_STRIPPED"   =>       0x0004,
    "LOCAL_SYMS_STRIPPED"   =>      0x0008,
    "AGGRESSIVE_WS_TRIM"   =>       0x0010,
    "LARGE_ADDRESS_AWARE"   =>     0x0020,
    "RESERVED"   =>                 0x0040,
    "BYTES_REVERSED_LO"   =>        0x0080,
    "32BIT_MACHINE"   =>   	        0x0100,
    "DEBUG_STRIPPED"   =>   	    0x0200,
    "REMOVABLE_RUN_ FROM_SWAP"   => 0x0400,
    "NET_RUN_FROM_SWAP"   =>  	    0x0800,
    "SYSTEM"   =>  	                0x1000,
    "DLL"   =>   	                0x2000,
    "UP_SYSTEM_ONLY"   =>   	    0x4000,
    "BYTES_REVERSED_HI"   =>   	    0x8000

);
