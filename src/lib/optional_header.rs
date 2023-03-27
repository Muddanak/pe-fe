use byteorder::{ByteOrder, LittleEndian};
use crate::optional_header::enums::{DLL_CHARACTERISTICS, MAGIC, SUBSYSTEM};
use crate::optional_header::structs::{OptHeader, OptHeaderDataDirectories, OptHeaderPE32Details, OptHeaderPE32PlusDetails};
use crate::utils::match_gen_in_map;

pub(crate) mod structs;
pub(crate) mod enums;

pub fn make_optional_header(data: &[u8], cursor: usize) -> OptHeader {
    let mut optheader = OptHeader::new();
    let mut cur: usize = cursor;

    optheader.MAGIC = LittleEndian::read_u16(&data[cur..cur+2]);
    optheader.DETAILS.MAGIC = match_gen_in_map(&MAGIC, optheader.MAGIC );
    cur += 2; //cur = 2
    optheader.MAJORLINKER = data[cur];
    cur += 1; //cur = 3
    optheader.MINORLINKER = data[cur];
    cur += 1; //cur = 4
    optheader.SIZEOFCODE = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4; //cur = 8
    optheader.SIZEOFINITDATA = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4; //cur = 12
    optheader.SIZEOFUNINITDATA = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4; //cur = 16
    optheader.BASEOFCODE = LittleEndian::read_u32(&data[cur..cur+4]);

    if optheader.MAGIC.eq(&0x20b) {
        let sizes: Vec<usize> = vec![8, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 2, 2, 8, 8, 8, 8, 4, 4];
        let mut wind = OptHeaderPE32PlusDetails::default();
        let mut veciter = sizes.iter();
        let mut tmpvec: Vec<String> = Vec::new();

        wind.IMAGEBASE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.SECTIONALIGNMENT = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.FILEALIGNMENT = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.MAJOROSVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.MINOROSVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.MAJORSUBVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.MINORSUBVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.WIN32VERSION = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFIMAGE = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEADERS = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.CHECKSUM = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();

        let comparator = LittleEndian::read_u16(&data[cur..cur+2]);
        for (charac, charac_id) in SUBSYSTEM.into_iter() {
            if (comparator & *charac_id).ne(&0)  | charac_id.eq(&0) {
                tmpvec.push(String::from(*charac));
            }
        }
        wind.SUBSYSTEM = tmpvec.join("|");
        tmpvec.clear();
        cur += veciter.next().unwrap();

        let comparator = LittleEndian::read_u16(&data[cur..cur+2]);
        for (charac, charac_id) in DLL_CHARACTERISTICS.into_iter() {
            if comparator & *charac_id != 0 {
                tmpvec.push(String::from(*charac));
            }
        }
        wind.DLLCHARACTERISTICS = tmpvec.join("|");
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKRESERVE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKCOMMIT = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPRESERVE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPCOMMIT = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.LOADERFLAGS = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.NUMBERRVASIDES = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();

        optheader.WINDETAILSPLUS = wind;

    } else if optheader.MAGIC.eq(&0x10b) {
        let sizes: Vec<usize> = vec![4,4, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 2, 2, 4, 4, 4, 4, 4, 4];
        let mut wind = OptHeaderPE32Details::default();
        let mut veciter = sizes.iter();
        let mut tmpvec: Vec<String> = Vec::new();

        optheader.BASEOFDATA = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.IMAGEBASE = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SECTIONALIGNMENT = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.FILEALIGNMENT = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.MAJOROSVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.MINOROSVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.MAJORSUBVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.MINORSUBVERSION = LittleEndian::read_u16(&data[cur..cur+2]);
        cur += veciter.next().unwrap();
        wind.WIN32VERSION = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFIMAGE = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEADERS = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.CHECKSUM = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();

        let comparator = LittleEndian::read_u16(&data[cur..cur+2]);
        for (charac, charac_id) in SUBSYSTEM.into_iter() {
            if (comparator & *charac_id).ne(&0)  | charac_id.eq(&0) {
                tmpvec.push(String::from(*charac));
            }
        }
        wind.SUBSYSTEM = tmpvec.join("|");
        tmpvec.clear();
        cur += veciter.next().unwrap();

        let comparator = LittleEndian::read_u16(&data[cur..cur+2]);
        for (charac, charac_id) in DLL_CHARACTERISTICS.into_iter() {
            if comparator & *charac_id != 0 {
                tmpvec.push(String::from(*charac));
            }
        }
        wind.DLLCHARACTERISTICS = tmpvec.join("|");
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKRESERVE = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFSTACKCOMMIT = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPRESERVE = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.SIZEOFHEAPCOMMIT = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.LOADERFLAGS = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();
        wind.NUMBERRVASIDES = LittleEndian::read_u32(&data[cur..cur+4]);
        cur += veciter.next().unwrap();

        optheader.WINDETAILS32 = wind;
    }

    {
        let sizes: Vec<usize> = vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8];
        let mut wind = OptHeaderDataDirectories::default();
        let mut veciter = sizes.iter();

        wind.EXPORTTABLE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.IMPORTTABLE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.RESOURCETABLE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.EXCEPTIONTABLE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.CERTTABLE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.BASERELOCATION = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.DEBUG = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.ARCHITECTURE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.GLOBALPTR = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.TLSTABLE = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.LOADCONFIG = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.BOUNDIMPORT = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.IAT = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.DELAYIMPDESC = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.CLRRUNTIME = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();
        wind.RESERVEDZERO = LittleEndian::read_u64(&data[cur..cur+8]);
        cur += veciter.next().unwrap();

        optheader.DATADIRECTORIES = wind;

    }

    println!("{cur}");
    optheader
}