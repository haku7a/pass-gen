use pyo3::prelude::*;

mod methods;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(methods::generate_password, m)?)?;
    Ok(())
}
