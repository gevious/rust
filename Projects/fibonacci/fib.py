# A python library comparing the result of FFI imported Rust code to native
# code. It assumes you have the library at /tmp/fibonacci.so
#
# The results are as expected, except for the fast implementation. This could
# be because the overhead of doing the system call takes far longer than the
# running of the python function.

from ctypes import cdll
import time
lib = cdll.LoadLibrary("/tmp/fibonacci.so")


def fib_count(n):
    if n < 1:
        return 0
    elif n < 3:
        return 1
    a, b = 1, 1
    f = 0
    for _ in range(3, n+1):
        f = a + b
        a = b
        b = f
    return f


def fib_recursive(n):
    if n < 1:
        return 0
    elif n < 2:
        return 1
    return fib_recursive(n-1) + fib_recursive(n-2)


h = {}


def fib_recursive_hash(n):
    if n < 1:
        h[n] = 0
        return 0
    elif n < 2:
        h[n] = 1
        return 1
    a = h.get(n-1, fib_recursive_hash(n-1))
    b = h.get(n-2, fib_recursive_hash(n-2))
    h[n] = a + b
    return a + b

fib_num = 35
print("All values count fibonacci for n={}".format(fib_num))
t = float(time.time())
lib.fibonacci_recursive(fib_num)
rt = float(time.time()) - t
print("Time Taken for recursive: {} seconds".format(rt))

t = float(time.time())
fib_recursive(fib_num)
pt = float(time.time()) - t
print("Time Taken for python recursive: {} seconds".format(pt))
print("Difference: {} milli seconds".format((pt - rt)*1000))
print("")

t = float(time.time())
lib.fibonacci_hash_recursive(fib_num)
rt = float(time.time()) - t
print("Time Taken for hash: {} seconds".format(rt))

t = float(time.time())
fib_recursive_hash(fib_num)
pt = float(time.time()) - t
print("Time Taken for python recursive hash: {} milli seconds".format(pt))
print("Difference: {} milli seconds".format((pt - rt)*1000))
print("")

t = float(time.time())
lib.fibonacci_fast(fib_num)
rt = float(time.time()) - t
print("Time Taken for fast: {} seconds".format(rt))

t = float(time.time())
fib_count(fib_num)
pt = float(time.time()) - t
print("Time Taken for python fast: {} seconds".format(pt))
print("Difference: {} milli seconds".format((pt - rt)*1000))
print("")

print("done!")
