use std::time::Instant;

fn main() {
    let before = Instant::now();

    let number_split: String = include_str!("pe-008-input.txt").chars().filter(|s| {!s.is_whitespace()}).collect();
    let mut max: u64 = 0;
    for i in 0..(number_split.len() - 13) {
        let window = &number_split[i..(i+13)];
        let product = window.chars().map(|s| {s.to_digit(10).unwrap() as u64}).product();
        if product > max {
            max = product;
        }
    }
    

    // print elapsed time
    println!("{:?}", before.elapsed());
    println!("{}", max);
}
