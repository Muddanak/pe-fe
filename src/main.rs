mod header;

use crate::header::enums::FileErrors;
use crate::header::functions::*;
use clap::Parser;
use std::fs::File;


#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let mut info: Vec<u8> = Vec::new();
    //let mut info_string = String::new();

    let _input_file = match File::open(args.filename) {
        Ok(success) => {
            info = get_first_kilobyte(success);
            Ok(())
        }
        Err(e) => Err(FileErrors::CouldNotOpenFile(e)),
    };

    match check_for_mz(&info[0..=1]) {
        Ok(()) => {
            let val: usize = match get_pe_offset(&info) {
                Ok(offset) => offset,
                Err(e) => return println!("Error type of {}", e),
            };

            let val_as_hex: usize = usize_to_hex(val);

            match verify_pe_header(&info[val_as_hex..=val_as_hex+3]) {
                Ok(()) => {
                    todo!()
                }
                Err(e) => println!("{}", e)
            }
        }
        Err(e) => println!("{e}")
    }

}
