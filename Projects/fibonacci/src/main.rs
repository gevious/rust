#![feature(test)]
use std::io;
use std::collections::HashMap;

/// This function returns the nth fibonacci number
///
/// Note: It only works up to about n=45, since after that it runs into
/// stack overflow issues.
pub fn fib_rec(n: u8) -> u64 {
    // Calculate the nth fibonacci number
    //
    // NOTE: This function only works up to about n=45, before running into
    // recursive issues
    if n < 1 {
        return 0
    }
    if n < 2 {
        return 1
    }
    fib_rec(n-1) + fib_rec(n-2)
}

/// This function uses a hash map to store previous numbers to try improve
/// performance for large numbers.
///
/// Note: This function works up to about n=94 after which we run into an
/// 'arithmetic overflow' error
pub fn fib_hash(n: u8) -> u64 {
    let mut map : HashMap<u8,u64> = HashMap::new();

    // This is the engine which recurses saving each value in the map
    fn f(map: &mut HashMap<u8,u64>, n: u8) -> u64 {
        let c = match map.get(&n) {
            Some(&number) => number,
            _ => 0
        };
        if c != 0 { return c }
        let m = match n {
            1 if n < 1   => 0,
            1 ... 2      => 1,
            _            => f(map, n-1) + f(map, n-2),
        };
        map.insert(n, m);
        m
    }
    f(&mut map, n)
}
/// This is an alternative hash approach from
/// http://stackoverflow.com/questions/30590341/recursive-fibonacci-with-hashmap-cache
/// in answer to my SO question.
pub fn fib_hash2(n: u8) -> u64 {
    // This is the engine which recurses saving each value in the map
    fn f(map: HashMap<u8,u64>, n: u8) -> (HashMap<u8, u64>, u64) {
        if let Some(&number) = map.get(&n) {
            return (map, number);
        }
        let (map, a) = f(map, n - 1);
        let (mut map, b) = f(map, n - 2);
        let res = a + b;
        map.insert(n, res);
        (map, res)
    }
    let mut map = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    map.insert(2, 1);
    f(map, n).1
}



/// This function counts up saving only the last two preceeding values. It 
/// can be converted into a generator for more versatility.
///
/// Note: This function also has an upper limit of n=94 because of the upper
/// bound of u64 (the same as `fib_hash`)
pub fn fib_count(n: u8) -> u64 {
    if n < 1 { return 0 }
    if n < 3 { return 1 }

    let (mut a, mut b) = (1, 1);
    let mut f = 0;
    for _ in 3..n+1 {
        f = a + b;
        a = b;
        b = f;
    }
    f
}

/// This is a proxy function to call our relevant fibonacci function
pub fn fib(n: u8) -> u64 {
    //fib_rec(n)
    //fib_hash(n)
    //fib_hash2(n)
    fib_count(n)
}

fn main() {
    println!("Fibonaci series calculator");
    println!("--------------------------");

    loop {
        // Ask for the number n and return the nth number in the series
        println!("Please enter a number (0 to exit)");
        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .ok()
            .expect("Failed to read the number");

        let n: u8 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number. Try again");
                continue;
            },
        };

        if n == 0 {
            println!("Quitting!");
            break;
        }

        print!("Calculating the fibonacci number....");

        // Not working for some reason!? We want to show the above line so
        // that there is at least some output while its calculating
        //io::stdout().flush().ok().expect("Couldn't flush");
        println!("{}", fib(n));
    }
}


extern crate test;
// NOTE: tests will only run with unstable version of rust. This is due to the
// test crate still being unstable.

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use test::black_box;

    #[test]
    fn test_fib_rec() {
        assert_eq!(fib_rec(3), 2);
        assert_eq!(fib_rec(5), 5);
        assert_eq!(fib_rec(7), 13);
    }

    #[test]
    fn test_fib_hash() {
        assert_eq!(fib_hash(3), 2);
        assert_eq!(fib_hash(5), 5);
        assert_eq!(fib_hash(7), 13);
    }

    #[test]
    fn test_fib_count() {
        assert_eq!(fib_count(3), 2);
        assert_eq!(fib_count(5), 5);
        assert_eq!(fib_count(7), 13);
    }

    #[bench]
    fn bench_5_fib_count(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(5);
            fib_count(n)
        });
    }

    #[bench]
    fn bench_20_fib_count(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(20);
            fib_count(n)
        });
    }

    #[bench]
    fn bench_40_fib_count(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(40);
            fib_count(n)
        });
    }

    #[bench]
    fn bench_5_fib_hash(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(5);
            fib_hash(n)
        });
    }

    #[bench]
    fn bench_20_fib_hash(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(20);
            fib_hash(n)
        });
    }

    #[bench]
    fn bench_40_fib_hash(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(40);
            fib_hash(n)
        });
    }
    #[bench]
    fn bench_5_fib_hash2(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(5);
            fib_hash2(n)
        });
    }

    #[bench]
    fn bench_20_fib_hash2(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(20);
            fib_hash2(n)
        });
    }

    #[bench]
    fn bench_40_fib_hash2(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(40);
            fib_hash2(n)
        });
    }

    #[bench]
    fn bench_5_fib_rec(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(5);
            fib_rec(n)
        });
    }

    #[bench]
    fn bench_20_fib_rec(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(20);
            fib_rec(n)
        });
    }

    #[bench]
    fn bench_40_fib_rec(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(40);
            fib_rec(n)
        });
    }
}

// we create some external functions so we can include this library from other
// code
#[no_mangle]
pub extern fn fibonacci_recursive(n: u8) -> u64 {
    fib_rec(n)
}

#[no_mangle]
pub extern fn fibonacci_hash_recursive(n: u8) -> u64 {
    fib_hash(n)
}

#[no_mangle]
pub extern fn fibonacci_fast(n: u8) -> u64 {
    fib_count(n)
}
