fn main() {
    let x = Some(1u8);
    if let Some(y) = x {
        println!("{:?}", y);
    }
}
