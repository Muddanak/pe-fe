use crate::optional_header::enums::{DLL_CHARACTERISTICS, MAGIC, SUBSYSTEM};
use crate::optional_header::structs::{
    OptHeader, OptHeaderDataDirectories, OptHeaderPE32Details, OptHeaderPE32PlusDetails,
};
use crate::utils::{match_gen_in_map, u64_to_u32};
use byteorder::{ByteOrder, LittleEndian};

pub(crate) mod enums;
pub(crate) mod structs;

/// Takes in the slice of the buffered data and where the offset for the optional header is located
///
/// Return a built optHeader based on the file opened, if it's a proper image
///
///  # Example
///
/// let optHead = make_optional_header(&buffer, 0x3c);
///
pub fn make_optional_header(data: &[u8], cursor: usize) -> (OptHeader, usize) {
    let mut optheader = OptHeader::new();
    let mut cur: usize = cursor;

    optheader.MAGIC = LittleEndian::read_u16(&data[cur..cur + 2]);
    optheader.DETAILS.MAGIC = match_gen_in_map(&MAGIC, optheader.MAGIC);
    cur += 2; //cur = 2
    optheader.MAJORLINKER = data[cur];
    cur += 1; //cur = 3
    optheader.MINORLINKER = data[cur];
    cur += 1; //cur = 4
    optheader.SIZEOFCODE = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4; //cur = 8
    optheader.SIZEOFINITDATA = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4; //cur = 12
    optheader.SIZEOFUNINITDATA = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4; //cur = 16
    optheader.ADDROFENTRYPOINT = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4; //cur = 20
    optheader.BASEOFCODE = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4; //cur = 24

    if optheader.MAGIC.eq(&0x20b) {
        let sizes: Vec<usize> = vec![
            8, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 2, 2, 8, 8, 8, 8, 4, 4,
        ];
        let mut wind = OptHeaderPE32PlusDetails::default();
        let mut veciter = sizes.iter();
        let mut tmpvec: Vec<String> = Vec::new();

        wind.IMAGEBASE = LittleEndian::read_u64(&data[cur..cur + 8]);
        cur += veciter.next().unwrap();
        wind.SECTIONALIGNMENT = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.FILEALIGNMENT = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.MAJOROSVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MINOROSVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MAJORIMGVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MINORIMGVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MAJORSUBVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MINORSUBVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.WIN32VERSION = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFIMAGE = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEADERS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.CHECKSUM = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SUBSYSTEM = match_gen_in_map(&SUBSYSTEM, LittleEndian::read_u16(&data[cur..cur + 2]));
        cur += veciter.next().unwrap();

        let comparator = LittleEndian::read_u16(&data[cur..cur + 2]);
        for (charac, charac_id) in DLL_CHARACTERISTICS.into_iter() {
            if (comparator & *charac_id).ne(&0) {
                tmpvec.push(String::from(*charac));
            }
        }

        wind.DLLCHARACTERISTICS = tmpvec.join(" | ");
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKRESERVE = LittleEndian::read_u64(&data[cur..cur + 8]);
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKCOMMIT = LittleEndian::read_u64(&data[cur..cur + 8]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPRESERVE = LittleEndian::read_u64(&data[cur..cur + 8]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPCOMMIT = LittleEndian::read_u64(&data[cur..cur + 8]);
        cur += veciter.next().unwrap();
        wind.LOADERFLAGS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.NUMBERRVASIDES = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();

        optheader.WINDETAILSPLUS = wind;
    } else if optheader.MAGIC.eq(&0x10b) {
        let sizes: Vec<usize> = vec![
            4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 2, 2, 4, 4, 4, 4, 4, 4,
        ];
        let mut wind = OptHeaderPE32Details::default();
        let mut veciter = sizes.iter();
        let mut tmpvec: Vec<String> = Vec::new();

        optheader.BASEOFDATA = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.IMAGEBASE = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SECTIONALIGNMENT = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.FILEALIGNMENT = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.MAJOROSVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MINOROSVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MAJORIMGVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MINORIMGVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MAJORSUBVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.MINORSUBVERSION = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += veciter.next().unwrap();
        wind.WIN32VERSION = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFIMAGE = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEADERS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.CHECKSUM = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SUBSYSTEM = match_gen_in_map(&SUBSYSTEM, LittleEndian::read_u16(&data[cur..cur + 2]));
        cur += veciter.next().unwrap();

        let comparator = LittleEndian::read_u16(&data[cur..cur + 2]);
        for (charac, charac_id) in DLL_CHARACTERISTICS.into_iter() {
            if comparator & *charac_id != 0 {
                tmpvec.push(String::from(*charac));
            }
        }
        wind.DLLCHARACTERISTICS = tmpvec.join(" | ");
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKRESERVE = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKCOMMIT = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPRESERVE = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPCOMMIT = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.LOADERFLAGS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();
        wind.NUMBERRVASIDES = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += veciter.next().unwrap();

        optheader.WINDETAILS32 = wind;
    }

    {
        let sizes: Vec<usize> = vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8];
        let mut wind = OptHeaderDataDirectories::default();
        let mut veciter = sizes.iter();

        wind.EXPORTTABLE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.IMPORTTABLE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.RESOURCETABLE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.EXCEPTIONTABLE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.CERTTABLE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.BASERELOCATION = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.DEBUG = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.ARCHITECTURE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.GLOBALPTR = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.TLSTABLE = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.LOADCONFIG = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.BOUNDIMPORT = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.IAT = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.DELAYIMPDESC = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.CLRRUNTIME = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();
        wind.RESERVEDZERO = u64_to_u32(LittleEndian::read_u64(&data[cur..cur + 8]));
        cur += veciter.next().unwrap();

        optheader.DATADIRECTORIES = wind;
    }

    (optheader, cur)
}
