mod header;

fn main() {
    let idnum = 0x0;
    let it = header::enums::MACHINE
        .into_iter()
        .find_map(|(&x, &y)| if y == idnum { Some(x) } else { None });

    if let Some(x) = it {
        println!("{}", x);
    }

    let it = header::enums::MACHINE2.get(&idnum);
    if let Some(x) = Some(it) {
        println!("{}", x.unwrap());
    }
}
