use fascii::is_ascii;
use std::time::Instant;

fn main() {
    // let mut bytes = vec![0xf0, 0x9f, 0x98, 0x81];
    let mut bytes = vec![];
    let mut long = vec![122; 60_000_000];
    bytes.append(&mut long); // 60 MB's
    println!("{}", bytes.len());
    let s = String::from_utf8(bytes).unwrap();

    let now = Instant::now();
    println!("{}", s.is_ascii());
    println!("{}", now.elapsed().as_micros());

    let now = Instant::now();
    println!("{}", is_ascii(&s));
    println!("{}", now.elapsed().as_micros());
}
