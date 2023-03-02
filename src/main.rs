mod header;

use crate::header::enums::FileErrors;
use crate::header::functions::*;
use clap::Parser;
use std::fs::File;
use byteorder::{ByteOrder, LittleEndian};


#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let mut info: Vec<u8> = Vec::new();
    //dbg!(&args.filename);

    let _input_file = match File::open(args.filename) {
        Ok(success) => {
            info = get_first_kilobyte(success);
            Ok(())
        }
        Err(e) => Err(FileErrors::CouldNotOpenFile(e)),
    };

    match check_for_mz(&info[0..=1]) {
        Ok(()) => {
            let val_for_pe = get_pe_offset(&info).unwrap();

            match verify_pe_header(&info[val_for_pe..=val_for_pe +3]).as_str() {
                "PE\0\0" => {
                    println!("Matched PE header");
                    let tmp = make_header_from_info(&info, val_for_pe);
                },
                _ => println!("{}", FileError::PEisNotHere)
            }
        }
        Err(e) => println!("{e}")
    }

}
