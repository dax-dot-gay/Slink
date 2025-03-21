use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};
use strfmt::strfmt;

use crate::JAVA_CONTAINER_BASE;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JavaVersion(pub u8);

impl JavaVersion {
    pub fn version(&self) -> u8 {
        self.0
    }

    pub fn image(&self) -> String {
        let mut map = HashMap::<String, String>::new();
        map.insert(String::from("version"), self.0.to_string());
        strfmt(JAVA_CONTAINER_BASE, &map).unwrap()
    }
}

impl Deref for JavaVersion {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for JavaVersion {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for JavaVersion {
    fn into(self) -> u8 {
        self.0
    }
}
