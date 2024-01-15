use std::time::Instant;

// fn collatz_counter(n: i32) -> i32 {
//     let mut n: u64 = n as u64;
//     for i in 0.. {
//         if n == 1 {
//             return i;
//         }
//         if n & 1 == 0 {
//             // n = n/2
//             n = n >> 1;
//         } else {
//             n = 3 * n + 1
//         }
//     }
//     return -1;
// }

struct CollatzNumber {
    index: i32,
    length: i32,
}

fn main() {
    let before = Instant::now();

    let mut number: CollatzNumber = CollatzNumber {
        index: 0,
        length: 0,
    };
    // compute length of collatz sequence for each number up to 1 million
    for i in 500001..1000000 {
        let mut n: u64 = i as u64;
        let mut length: i32 = 0;
        for j in 0.. {
            if n == 1 {
                length = j;
                break;
            }
            if n & 1 == 0 {
                // n = n/2
                n = n >> 1;
            } else {
                n = 3 * n + 1
            }
        }
        if number.length < length {
            number.index = i;
            number.length = length;
        }
    }

    // print elapsed time
    println!("{:?}", before.elapsed());
    println!("{}", number.index);
}
