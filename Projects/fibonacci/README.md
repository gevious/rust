The Fibonacci sequence is a set of positive integers that follow the following
pattern

    1, 1, 2, 3, 5, 8, 13, 21, 34 ...

The pattern is essentially:

    fn(n) = 1 where n < 2
    f(n) = f(n-1) + f(n-2)

This project contains the code for calculating the nth fibonacci number in
various ways.

# Sample Output

## Benchmarks

    test tests::bench_20_fib_count ... bench:        34 ns/iter (+/- 2)
    test tests::bench_20_fib_hash  ... bench:      1665 ns/iter (+/- 275)
    test tests::bench_20_fib_hash2 ... bench:      2301 ns/iter (+/- 207)
    test tests::bench_20_fib_rec   ... bench:     57140 ns/iter (+/- 1111)
    test tests::bench_40_fib_count ... bench:        60 ns/iter (+/- 3)
    test tests::bench_40_fib_hash  ... bench:      4358 ns/iter (+/- 212)
    test tests::bench_40_fib_hash2 ... bench:      5546 ns/iter (+/- 49)
    test tests::bench_40_fib_rec   ... bench: 880999939 ns/iter (+/- 65038240)
    test tests::bench_5_fib_count  ... bench:        15 ns/iter (+/- 5)
    test tests::bench_5_fib_hash   ... bench:       447 ns/iter (+/- 52)
    test tests::bench_5_fib_hash2  ... bench:       597 ns/iter (+/- 92)
    test tests::bench_5_fib_rec    ... bench:        40 ns/iter (+/- 1)

## Python comparison

    All values count fibonacci for n=35
    Time Taken for recursive: 0.09705948829650879 seconds
    Time Taken for python recursive: 4.83321213722229 seconds
    Difference: 4736.152648925781 milli seconds
    
    Time Taken for hash: 0.00021314620971679688 seconds
    Time Taken for python recursive hash: 12.549077987670898 milli seconds
    Difference: 12548.864841461182 milli seconds

    Time Taken for fast: 4.4345855712890625e-05 seconds
    Time Taken for python fast: 1.4066696166992188e-05 seconds
    Difference: -0.030279159545898438 milli seconds
    
