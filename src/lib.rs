use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyValueError;
use serde_json::Value;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};


#[pyfunction]
fn flatten_dict(json_string: String, sep: Option<String>) -> PyResult<String> {
    let separator = sep.unwrap_or_else(|| "_".to_string());
    let json_data: Value = serde_json::from_str(&json_string)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON: {}", e)))?;
    let flat_data = flatten(&json_data, String::new(), &separator);

    let flat_json = serde_json::to_string(&flat_data)
        .map_err(|e| PyValueError::new_err(format!("Failed to stringify JSON: {}", e)))?;
    Ok(flat_json)
}


#[pymodule]
fn kangajo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(flatten_dict))?;

    Ok(())
}

fn flatten(obj: &serde_json::Value, parent_key: String, sep: &str) -> BTreeMap<String, serde_json::Value> {
    let mut result = BTreeMap::new();

    match obj {
        serde_json::Value::Array(arr) => {
            let sub_results: Vec<_> = arr
                .par_iter()
                .enumerate()
                .map(|(i, value)| {
                    let new_key = if parent_key.is_empty() {
                        i.to_string()
                    } else {
                        format!("{}{}{}", parent_key, sep, i)
                    };
                    flatten(value, new_key, sep)
                })
                .collect();

            for sub_result in sub_results {
                for (k, v) in sub_result {
                    result.insert(k, v);
                }
            }
        }
        serde_json::Value::Object(map) => {
            let hashmap: HashMap<_, _> = map.iter().collect();
            let sub_results: Vec<_> = hashmap
                .par_iter()
                .map(|(k, v)| {
                    let new_key = if parent_key.is_empty() {
                        k.to_string()
                    } else {
                        format!("{}{}{}", parent_key, sep, k)
                    };
                    flatten(v, new_key, sep)
                })
                .collect();

            for sub_result in sub_results {
                for (k, v) in sub_result {
                    result.insert(k, v);
                }
            }
        }
        _ => {
            result.insert(parent_key.clone(), obj.clone());
        }
    }

    result
}