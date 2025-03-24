use std::{cmp::Ordering, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Into<String> for Version {
    fn into(self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(".");
        let major = parts
            .next()
            .ok_or(Error::value_error(
                s.clone(),
                "Expected at least one version part",
            ))?
            .parse::<u32>()
            .or_else(|e| Err(Error::value_error(s.clone(), e)))?;

        let minor = parts
            .next()
            .unwrap_or("0")
            .parse::<u32>()
            .or_else(|e| Err(Error::value_error(s.clone(), e)))?;
        let patch = parts
            .next()
            .unwrap_or("0")
            .parse::<u32>()
            .or_else(|e| Err(Error::value_error(s.clone(), e)))?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match Ord::cmp(&self.major, &other.major) {
            Ordering::Equal => match Ord::cmp(&self.minor, &other.minor) {
                Ordering::Equal => 
            }
        }
    }
}