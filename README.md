# rustpy_gpu
Playing with PyO3 to hopefully GPU accelerate matmul ops from Python in Rust.

# Libraries used
- [PyO3](https://github.com/PyO3/pyo3), Rust bindings for Python.
- [Maturin](https://github.com/PyO3/maturin), build and install the Rust code as Python module locally.
- [Vulkano](https://docs.rs/vulkano/0.12.0/vulkano/), Rust wrapper for Vulkan API (GPU).
- [Arrow](https://docs.rs/arrow/latest/arrow/), native Rust implementation of Apache Arrow.
- [PyArrow](https://arrow.apache.org/docs/python/index.html), Python module for Apache Arrow in-memory data.
- [OpenCL3](https://docs.rs/opencl3/latest/opencl3/), A Rust implementation of the Khronos OpenCL API (GPU/Parallelization).

# How to start developing?..
```
$ python -m venv .env
$ source .env/bin/activate
$ pip install maturin
...
$ maturin init
```
Now choose the PyO3 bindings and wait for it to set up the Rust crate directory structure. In `src/lib.rs` you will find on of the Rust files that you can implement the native Rust functions in, that will be callable from Python!

```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

To build your Rust implementations run `maturin develop` and the package will be installed into the Python virtualenv that you previously created.

# How does it all work?
Hopefully `Apache Arrow` will be the solution that ties the knot on GPU accelerated matmul ops in Python via Rust. The native Rust implementation of Arrow is a hard-requirement for this, as well as PyO3 together with libc. The Python dependencies is at the moment a bit more undefined, but it seems like all that will be needed is numpy, PyArrow and maturin/PyO3.

For example:
```python
# inspiration from https://github.com/jhoekx/python-rust-arrow-interop-example
import pyarrow
import my_local_rust_lib
import numpy as np

data = pyarrow.array(
    np.random.uniform(
        low=-1.0, 
        high=1.0, 
        size=(64, 128)
    )
)
result = my_local_rust_lib.example_function(data)
...
```
The way this should work is that the PyArrow module creates an in-memory array of the already initialized numpy array and then sends the pointer in memory to the pre-compiled Rust function. It should then either operate on the Arrow array in memory or return a newly created array to the Python module. Have to test this and see how it works...

# License
All code that I have written is under an Apache-2.0 styled license, see [LICENSE](https://github.com/willeagren/rustpy_gpu/blob/main/LICENSE) for specific information.

