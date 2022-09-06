#[allow(unused_imports)]
use numpy::ndarray::{ArrayD, ArrayViewD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

#[pymodule]
fn rustpy_gpu(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    // double the values of the ndarray
    fn double(arr: ArrayViewD<'_, f32>) -> ArrayD<f32> {
        2.0 * &arr
    }

    #[pyfn(m)]
    #[pyo3(name="double")]
    fn double_py<'py>(py: Python<'py>, arr: PyReadonlyArrayDyn<'_, f32>) -> &'py PyArrayDyn<f32> {
        let arr = arr.as_array();
        double(arr).into_pyarray(py)
    }

    // bit-wise multiplication of two ndarray's
    fn multiply(u: ArrayViewD<'_, f32>, v: ArrayViewD<'_, f32>) -> ArrayD<f32> {
       &u * &v 
    }

    #[pyfn(m)]
    #[pyo3(name="multiply")]
    fn multiply_py<'py>(py: Python<'py>, u: PyReadonlyArrayDyn<'_, f32>, v: PyReadonlyArrayDyn<'_, f32>) -> &'py PyArrayDyn<f32> {
        let u = u.as_array();
        let v = v.as_array();
        multiply(u, v).into_pyarray(py)
    }

    Ok(())
}