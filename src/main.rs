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
            let val_for_pe = get_pe_header(&info);
            let tmp = make_header_from_info(&info, val_for_pe);
            println!("{}", tmp)
        }
        Err(e) => println!("{e}"),
    }
}
