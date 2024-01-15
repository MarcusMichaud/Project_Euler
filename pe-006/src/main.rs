use std::time::Instant;

fn main() {
    // start timer
    let before = Instant::now();

    let edge = 100;

    // (1..100+1).sum()
    // Gausses trick
    let sum_range: i32 = edge * (edge + 1) / 2;
    let square_sum = sum_range * sum_range;

    // (1..100+1).map(|s| {s*s}).sum()
    // closed form is
    // x^3/3 + x^2/2 + x/6, or alternately: x(2x+1)(x+1)/6
    let sum_squares: i32 = edge * (2 * edge + 1) * (edge + 1) / 6;

    // print elapsed time
    println!("{:?}", before.elapsed());
    println!("{}", square_sum - sum_squares);
}
