use std::fs::File;
use std::io::Read;

pub fn get_first_kilobyte(mut filename: File) -> Vec<u8> {
    //let mut chunk: Vec<u8> = vec![0u8; 0x400];
    let mut chunk = [0; 0x400];

    if let Ok(x) = filename.read_exact(&mut chunk) {
        //(Vec::from(chunk.map(|x| x as char)), chunk.into_iter().map(|x| x as char).collect::<String>())
        Vec::from(chunk)
    } else {
        //(Vec::from(chunk.map(|x| x as char)), String::from("None"))
        Vec::from(chunk)
    }
}