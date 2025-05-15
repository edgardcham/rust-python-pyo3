import time

from rust_python_fib import fibonacci_rust


def fibonacci_python(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci_python(n - 1) + fibonacci_python(n - 2)


def benchmark(n: int) -> None:
    start_time = time.time()
    py_result = fibonacci_python(n)
    py_time = time.time() - start_time

    start_time = time.time()
    rust_result = fibonacci_rust(n)
    rust_time = time.time() - start_time

    print(f"\nResults for Fibonacci({n}):")
    print(f"Python result: {py_result}")
    print(f"Python time: {py_time:.6f} seconds")
    print(f"Rust result: {rust_result}")
    print(f"Rust time: {rust_time:.6f} seconds")
    print(f"Rust is {py_time / rust_time:.2f}x faster than Python")


if __name__ == "__main__":
    for n in [20, 30, 35, 40]:
        benchmark(n)
