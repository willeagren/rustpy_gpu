# rustpy_gpu
Playing with PyO3 to hopefully GPU accelerate matmul ops from Python in Rust.

# Libraries/Codebases used
- [PyO3](https://github.com/PyO3/pyo3), Rust bindings for Python.
- [Maturin](https://github.com/PyO3/maturin), build and install the Rust code as Python module locally.
- [Vulkano](https://docs.rs/vulkano/0.12.0/vulkano/), Rust wrapper for Vulkan API (GPU).
- [Arrow](https://docs.rs/arrow/latest/arrow/), native Rust implementation of Apache Arrow.
- [PyArrow](https://arrow.apache.org/docs/python/index.html), Python module for Apache Arrow in-memory data.

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

# License
All code that I have written is under an Apache-2.0 styled license, see [LICENSE](https://github.com/willeagren/rustpy_gpu/blob/main/LICENSE) for specific information.
