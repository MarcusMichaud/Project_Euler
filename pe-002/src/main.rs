#![warn(unused_variables)]
#![feature(more_float_constants)]
use std::f64::consts::PHI;
use std::time::Instant;


fn main() {
    // start timer
    let before = Instant::now();

    // limit set by project euler
    let limit = 4000000;
    // store coefficient of equation
    let one_over_root_five = 1_f64 / (5_f64).sqrt();

    let mut count: i32 = 0;
    for i in 0.. {
        // nth fibonnacci number is roughly equal to (PHI)^n/sqrt(5)
        // PHI is the golden ratio
        let n = (f64::powi(PHI, i) * one_over_root_five).round() as i32;
        // if n is over the limit, stop the loop
        if n > limit {
            break;
        }
        // else if it's even, add it to the count
        else if (n & 1) == 0 {
            count += n;
        }
    }
    // print elapsed time
    println!("{:?}", before.elapsed());

    // // fibonnacci sequence generator; c is the next number in sequence
    // // sequence [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...]
    // let limit = 4000000;
    // let mut a = 0;
    // let mut b = 1;
    // let mut c;
    // let mut count2 = 0;
    // while b < limit {
    //     c = a + b;
    //     a = b;
    //     b = c;
    //     if (c & 1) == 0 {
    //         count2 += c;
    //     }
    // }
}
