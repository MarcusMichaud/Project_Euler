use std::time::Instant;

fn main() {
    let before = Instant::now();
    let numbers = include_str!("pe-013-input.txt")
        .as_bytes()
        .iter()
        .map(|s| s % 48)
        .collect::<Vec<u8>>();

    // print elapsed time
    println!("{:?}", before.elapsed());
    // println!("{:?}", numbers);
}
