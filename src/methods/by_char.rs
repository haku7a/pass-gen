use pyo3::prelude::*;
use rand::Rng;
use rand::rngs::OsRng;

#[pyfunction]
pub fn generate_password(length: usize) -> PyResult<String> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789\
                    !@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut rng: OsRng = OsRng;

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    Ok(password)
}
