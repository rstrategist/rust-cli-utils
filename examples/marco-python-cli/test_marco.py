# test_marco.py
# -----------------------------------------------------------------------------
# Example Python script that imports and uses the Rust extension module
# `marco_python_cli` built with PyO3. Demonstrates calling the Rust
# `marco_python` function from Python.
# -----------------------------------------------------------------------------

from marco_python_cli import marco_python

def test_marco():
    assert marco_python("marco") == "python"
    assert marco_python("not_marco") == "no python"

if __name__ == "__main__":
    print(marco_python("marco"))      # Should print "python"
    print(marco_python("not_marco"))  # Should print "no python"