Linux Framebuffer Display for CircuitPython
===========================================
### By Carsten Thue-Bludworth, 2023

`framebuffer-display` is CircuitPython-compatible `displayio` display that targets the Linux Framebuffer. The package is written in Rust and bindings to Python are produced with `pyo3`. This allows CircuitPython scripts to display a `displayio` GUI directly to the framebuffer device in the case that the host system does not have display server/compositor setup.

### Compilation
The `maturin` tool is used to generate the `pyo3` python bindings from the Rust source code. Use `cargo install maturin` to install the tool, and then generate the binding with `maturin develop`.
The display class is defined in the Python module `framebuffer-display`.

### Usage
* Create and activate a Python virtual environment, and install the needed dependencies with `pip install -r requirements.txt`
* In a TTY, run an example with `python examples/<example>.py`

### Limitations and Improvements
* TODO - host on PyPi
