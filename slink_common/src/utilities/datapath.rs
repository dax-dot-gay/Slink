use serde::de::DeserializeOwned;
use serde_json::{Value, from_value, to_string_pretty};
use serde_json_path::JsonPath;

use crate::{Error, Res};

pub fn get_at_path<T: DeserializeOwned>(path: impl AsRef<str>, data: &Value) -> Res<Vec<T>> {
    let datapath = JsonPath::parse(path.as_ref()).expect("valid JSONPath specifier");
    let raw = datapath.query(data).all();

    let mut results: Vec<T> = Vec::new();
    for result in raw {
        results.push(from_value::<T>(result.clone()).or_else(|e| {
            Err(Error::DataPathError {
                path: path.as_ref().to_string(),
                data: to_string_pretty(data).unwrap_or(String::from("UNPARSEABLE")),
                reason: e.to_string(),
            })
        })?);
    }

    Ok(results)
}

pub fn get_one_at_path<T: DeserializeOwned>(path: impl AsRef<str>, data: &Value) -> Res<T> {
    let datapath = JsonPath::parse(path.as_ref()).expect("valid JSONPath specifier");
    let raw = datapath.query(data).exactly_one().or_else(|_| {
        Err(Error::DataPathError {
            path: path.as_ref().to_string(),
            data: to_string_pretty(data).unwrap_or(String::from("UNPARSEABLE")),
            reason: String::from("No records found matching path"),
        })
    })?;

    from_value::<T>(raw.clone()).or_else(|e| {
        Err(Error::DataPathError {
            path: path.as_ref().to_string(),
            data: to_string_pretty(data).unwrap_or(String::from("UNPARSEABLE")),
            reason: e.to_string(),
        })
    })
}
