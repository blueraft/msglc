use pyo3::prelude::*;

#[pymodule]
fn _msglc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__name__", "_msglc")?;
    Ok(())
}
