mod header;

use std::fs::File;
use std::io::{BufReader};
use clap::Parser;
use crate::header::functions::get_first_kilobyte;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {

    let args = Args::parse();
    let mut info: Vec<u8> = Vec::new();
    let input_file= File::open(args.filename);

    if let Ok(file_open) = input_file {
        info = get_first_kilobyte(file_open)
    }

    let val = info[0x3c] as usize;
    let val2 = info.get(val);

    println!("{}", val2.unwrap());

    /*if info.as_mut_slice()[0..=1].iter()
        .map(|&x| x as char)
        .collect::<String>()
        .contains("MZ")
    {
        let val = *info.clone().get(0x3c).unwrap();

        let val2 = info.get(val).unwrap();


    }*/

    //let info_string :String = info.iter().map(|x| *x as char).collect();
    //let disp: Vec<char> = info.into_iter().map(|x| x as char).collect();
    //println!("{:?}, {}", info, info_string);
    /*if info[0..=2].into_iter().map(|x| *x as char).collect::<String>().contains("MZ") {

    }*/
    /*let idnum = 0x0;
    let machine_type = header::enums::MACHINE
        .into_iter()
        .find_map(|(&x, &y)| if y == idnum { Some(x) } else { None });

    if let Some(x) = machine_type {
        println!("{}", x);
    }*/



}
