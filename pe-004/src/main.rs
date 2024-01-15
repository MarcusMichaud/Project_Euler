use std::time::Instant;

fn is_palindromic(n: i32) -> bool {
    let mut reversed: i32 = 0;
    let mut new = n;
    while new > 0 {
        reversed = 10 * reversed + new % 10;
        new = new / 10;
    }
    return reversed == n;
}

fn main() {
    // start timer
    let before = Instant::now();

    let mut largest_palindrome = 0;
    let mut b: i32;
    let mut db: i32;
    for a in (100..1000).rev() {
        // 6 digit palindromic numbers have to be divisible by 11
        // if a is divisible by 11, then b doesn't have to be, and it can decrement by 1
        // but if a is not, then b has to be, so it has to start at the highest multiple of 11,
        //     and decrement by 11 so that it's always a multiple of 11
        if a % 11 == 0 {
            b = 999;
            db = 1;
        } else {
            b = 990;
            db = 11
        }
        while b >= a {
            if a * b <= largest_palindrome {
                break;
            } else if is_palindromic(a * b) {
                largest_palindrome = a * b;
            }
            b -= db;
        }
    }

    // print elapsed time
    println!("{:?}", before.elapsed());
    println!("{}", largest_palindrome);
}
