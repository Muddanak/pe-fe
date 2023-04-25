use clap::{arg, Parser};
use std::fs::File;
use std::io::{BufReader, Read};
use std::{io, process};
use druid::{AppLauncher, LocalizedString, WindowDesc};

use pe_fe::sections_header::{make_section_header, print_section_headers};
use pe_fe::{lib as pefelib, show_headers};
use pe_fe::pefegui::build_root_widget;
use pe_fe::pefegui::structs::PefeApp;
use pefelib::dos_header::{check_for_mz, make_dos_header, print_rich_sha256_hash};

use pefelib::coff_header::make_coff_header;
use pefelib::optional_header::make_optional_header;

const WINDOW_NAME: &str = "Pe-fe - A PE File Analyzer";

#[derive(Parser)]
#[command(author = "Mudd", version, long_about = None)]
#[command(about = "A PE file analyzer written in Rust")]
#[command(arg_required_else_help = true, propagate_version = true)]
struct Args {
    ///Filename to analyze
    filename: String,

    #[arg(short, long="rich")]
    ///Show the 'Rich' header hash
    rich_hash: bool,

    #[arg(short, long="section")]
    ///Show the Sections header
    section_header: bool,

    ///Show all header info parsed
    #[arg(short = 'a', long = "all")]
    show_all: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut reader = BufReader::new(File::open(args.filename)?);
    let mut buffer = [0; 0x800];
    let _size_read = reader.read(&mut buffer)?;

    let mz_offset = match check_for_mz(&buffer) {
        Ok(offset) => offset,
        Err(e) => {
            println!("{e}");
            process::exit(1)
        }
    };

    let header_dos = make_dos_header(&buffer, mz_offset);

    let mut cursor = header_dos.0.pe_offset + 4;

    let header_coff = make_coff_header(&buffer, cursor);

    cursor += 20;

    let (header_opt, cursor) = make_optional_header(&buffer, cursor);

    let secheader = make_section_header(&buffer, cursor, header_coff.HE_SECTIONS as usize);

    show_headers(&header_dos.0, &header_coff, &header_opt);

    if args.section_header.eq(&true) || args.show_all {
        print_section_headers(&secheader);
    }

    if header_dos.0.has_rich && (args.rich_hash.eq(&true) || args.show_all) {
        print_rich_sha256_hash(&header_dos.0);
    }

    let appstate = PefeApp {
        name: "PE File Analyzer".to_string(),
        dos_head: header_dos.0,
        current_tab: 0,
    };

    let main_window = WindowDesc::new(build_root_widget())
        .title(LocalizedString::new(WINDOW_NAME))
        .window_size((600.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(appstate)
        .expect("Failed to launch application");

    Ok(())

}
