# PyCalc-CLI

A fast and robust command-line calculator powered by Rust and Python.  
Rust provides high-performance calculation functions, exposed to Python via PyO3, and a user-friendly CLI is built using Python Fire.

## Features

- **Basic arithmetic:** add, subtract, multiply, divide
- **Advanced operations:** square root, exponentiation
- **Robust error handling:** input validation, division by zero, negative square root
- **Easy CLI usage:** clear help text, intuitive commands
- **Unit tested:** with pytest for reliability
- **Performance:** Rust backend for fast calculations

## Installation

1. **Build the Rust Python extension:**

   ```bash
   cd /workspaces/rust-with-python/pycalc-cli
   pip install maturin
   maturin develop
   ```

2. **Install Python dependencies:**
   ```bash
   pip install fire pytest
   ```

## Usage

```bash
python3 pycalc-cli/calc.py add 2 3
python3 pycalc-cli/calc.py subtract 10 4
python3 pycalc-cli/calc.py multiply 7 8
python3 pycalc-cli/calc.py divide 20 5
python3 pycalc-cli/calc.py sqrt 16
python3 pycalc-cli/calc.py power 2 8
python3 pycalc-cli/calc.py --help
```

## Testing

Run all unit tests:

```bash
pytest pycalc-cli/test_calc.py
```

## Example Output

```bash
$ python3 pycalc-cli/calc.py add 2 3
5

$ python3 pycalc-cli/calc.py sqrt -1
ValueError: Cannot take square root of negative number.
```

## Project Structure

- `calc.py` — Python CLI
- `src/lib.rs` — Rust backend (PyO3)
- `test_calc.py` — Unit tests
- `README.md` — This file

## License

MIT
