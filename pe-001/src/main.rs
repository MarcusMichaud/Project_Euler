use std::time::Instant;

// sum of all natural numbers below "a number" that are multiples of "the second number"
fn sum_multiples(cap: i32, m: i32) -> i32 {
    // cap - 1, because it can't include the cap
    let n = ((cap - 1) / m) * m;
    return n * (n / m + 1) / 2;
}

fn main() {
    // start timer
    let before = Instant::now();

    let limit = 1000;

    // math
    // sum of both groups of (multiples of 3) and (multiples of 5), minus the overlap
    let result = sum_multiples(limit, 3) + sum_multiples(limit, 5) - sum_multiples(limit, 15);
    
    // // built in rust iterators and methods
    // // sum of both groups of (multiples of 3) and (multiples of 5), minus the overlap
    // let _result = (0..limit).step_by(3).sum::<i32>() + (0..limit).step_by(5).sum::<i32>()
    //     - (0..limit).step_by(15).sum::<i32>();
    
    // print elapsed time
    println!("{:?}", before.elapsed());
    println!("{}", result);
}
