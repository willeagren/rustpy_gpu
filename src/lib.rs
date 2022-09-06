#[allow(unused_imports, unused_variables)]
use pyo3::{
    pymodule, pyfunction,  wrap_pyfunction, 
    PyResult, Python, PyObject, types::PyModule,
};

use arrow::array::{Int32Array, make_array_from_raw};
use arrow::ffi;

#[pyfunction]
fn get_arrow_array(py: Python, obj: PyObject) -> PyResult<()> {
    let (arr_ptr, schema_ptr) = 
        ffi::ArrowArray::into_raw(unsafe { ffi::ArrowArray::empty() });
    Ok(())
}

#[pymodule]
fn rustarrow(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_arrow_array))?;
    Ok(())
}