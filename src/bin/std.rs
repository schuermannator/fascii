fn main() {
    let mut bytes = vec![];
    // let mut bytes = vec![0xf0, 0x9f, 0x98, 0x81];
    let mut long = vec![122; 60_000_000];
    bytes.append(&mut long); // 60 MB's
    println!("{}", bytes.len());
    let s = String::from_utf8(bytes).unwrap();
    println!("{}", s.is_ascii());
}
