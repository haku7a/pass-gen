use pyo3::prelude::*;

#[pyfunction]
fn generate_password() -> PyResult<String> {
    Ok("secure_pass_123".to_string())
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_password, m)?)?;
    Ok(())
}
