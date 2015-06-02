use std::io;
use std::collections::HashMap;

/// This function returns the nth fibonacci number
///
/// Note: It only works up to about n=45, since after that it runs into
/// stack overflow issues.
pub fn fib_basic(n: u32) -> u64 {
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
    fib(n-1) + fib(n-2)
}

/// This function uses a hash map to store previous numbers to try improve
/// performance for large numbers.
///
/// Note: This function works up to about n=94 after which we run into an
/// 'arithmetic overflow' error
pub fn fib_hash(n: u32) -> u64 {
    // we can improve this with match, but for now the simple case first
    let mut map : HashMap<u32,u64> = HashMap::new();
    for i in 1..n+1 {
        if i <= 2 {
            map.insert(i, 1);
            continue;
        }
        // get the previous two numbers and save the sum into the map
        let n1 = match map.get(&(i-1)) {
            Some(&number) => number,
            _ => 0
        };
        let n2 = match map.get(&(i-2)) {
            Some(&number) => number,
            _ => 0
        };
        map.insert(i, n1 + n2);
    };
    match map.get(&n) {
        Some(&number) => number,
        _ => 0
    }
}

/// This is a proxy function to call our relevant fibonacci function
pub fn fib(n: u32) -> u64 {
    //fib_basic(n)
    fib_hash(n)
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

        let n: u32 = match n.trim().parse() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_basic() {
        assert_eq!(fib_basic(3), 2);
        assert_eq!(fib_basic(5), 5);
        assert_eq!(fib_basic(7), 13);
    }

    #[test]
    fn test_fib_hash() {
        assert_eq!(fib_hash(3), 2);
        assert_eq!(fib_hash(5), 5);
        assert_eq!(fib_hash(7), 13);
    }
}
