use pyo3::prelude::*;
use rand::Rng;
use rand::rngs::OsRng;

const WORDS: &str = include_str!("../words.txt");

#[pyfunction]
pub fn generate_passphrase(word_count: usize) -> PyResult<String> {
    let wordlist: Vec<&str> = WORDS.lines().filter(|s| !s.is_empty()).collect();
    if wordlist.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Wordlist is empty",
        ));
    }

    let mut rng = OsRng;

    let mut chosen: Vec<String> = (0..word_count)
        .map(|_| wordlist[rng.gen_range(0..wordlist.len())].to_string())
        .collect();

    let num_chosen = chosen.len();
    if num_chosen > 0 {
        let word_idx = rng.gen_range(0..num_chosen);

        if let Some(word_to_modify) = chosen.get_mut(word_idx) {
            let positions: Vec<(usize, char)> = word_to_modify
                .char_indices()
                .filter(|(_, c)| c.is_ascii_alphabetic() && *c != 'i' && *c != 'I')
                .collect();

            let num_positions = positions.len();
            if num_positions > 0 {
                let char_lookup_idx = rng.gen_range(0..num_positions);
                if let Some(&(char_idx, c)) = positions.get(char_lookup_idx) {
                    let upper = c.to_uppercase().to_string();
                    word_to_modify.replace_range(char_idx..char_idx + c.len_utf8(), &upper);
                }
            }
        }
    }

    Ok(chosen.join("-"))
}
