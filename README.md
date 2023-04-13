Linux Framebuffer Display for CircuitPython
===========================================
#### By Carsten Thue-Bludworth, 2023

`displayio-fbdev` is CircuitPython-compatible displayio `Display` that targets the Linux fbdev graphics driver. The package is written in Rust and bindings to Python are produced with `pyo3`. This allows CircuitPython scripts to display a displayio GUI directly to the framebuffer device in the case that the host system does not have a display server/compositor setup.

### Compilation
The `maturin` tool is used to generate the `pyo3` python bindings from the Rust source code. Use `cargo install maturin` to install the tool, and then generate the bindings with `maturin develop`.
The `framebuffer-display` module provides the `Framebuffer` class used in the script.

### Usage
* Create and activate a Python virtual environment, and install the needed dependencies with `pip install -r requirements.txt`
* In a TTY, run an example with `python examples/<example>.py`

### Limitations and Improvements
* TODO - test on various devices (pinephone, cyberdeck, steamdeck)
* TODO - host on PyPi
