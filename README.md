# Rust vs Python Fibonacci Performance Comparison

This project demonstrates the performance difference between Rust and Python implementations of the Fibonacci sequence calculator using PyO3 for Rust-Python bindings.

## Project Structure

```
.
├── src/
│   └── lib.rs         # Rust implementation with PyO3 bindings
├── python/
│   └── benchmark.py   # Python implementation and benchmarking code
├── Cargo.toml         # Rust package configuration
└── pyproject.toml     # Python package configuration
```

## Features

- Recursive Fibonacci implementation in both Rust and Python
- PyO3 bindings to call Rust from Python
- Benchmark comparison showing performance differences
- Type hints and stubs for better IDE support

## Setup

1. Make sure you have Rust and Python installed
2. Install development tools:
   ```bash
   uv venv
   source .venv/bin/activate
   uv pip install -e .
   ```

## Running the Benchmark

```bash
python python/benchmark.py
```

Example output:
```
Results for Fibonacci(20):
Python result: 6765
Python time: 0.001434 seconds
Rust result: 6765
Rust time: 0.000065 seconds
Rust is 22.03x faster than Python

Results for Fibonacci(30):
Python result: 832040
Python time: 0.086604 seconds
Rust result: 832040
Rust time: 0.003627 seconds
Rust is 23.88x faster than Python
```

## Development

- Build Rust code: `uv run maturin develop`
- Run type checks: `poe typecheck`
- Run linting: `poe lint`
- Run all checks: `poe check`

## Why This Example?

This project demonstrates:
1. How to use PyO3 to create Python bindings for Rust code
2. Performance comparison between Rust and Python
3. Proper project structure for Rust-Python hybrid projects
4. Type safety and IDE support setup

# Sample Run Results

```
Results for Fibonacci(20):
Python result: 6765
Python time: 0.000639 seconds
Rust result: 6765
Rust time: 0.000043 seconds
Rust is 14.81x faster than Python

Results for Fibonacci(30):
Python result: 832040
Python time: 0.058233 seconds
Rust result: 832040
Rust time: 0.003371 seconds
Rust is 17.27x faster than Python

Results for Fibonacci(35):
Python result: 9227465
Python time: 0.578205 seconds
Rust result: 9227465
Rust time: 0.036547 seconds
Rust is 15.82x faster than Python

Results for Fibonacci(40):
Python result: 102334155
Python time: 6.444686 seconds
Rust result: 102334155
Rust time: 0.420747 seconds
Rust is 15.32x faster than Python
```