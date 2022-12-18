use std::collections::BTreeMap;

use crate::col::Column;

use opentelemetry::Value;

/// Contains many columns.
pub struct Row {
    columns: Vec<Column>,
}

impl Row {
    pub fn as_columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn to_map(&self) -> BTreeMap<String, Value> {
        let i = self.as_columns().iter().map(|c: &Column| {
            let key: &str = c.as_name();
            let val: &Value = c.as_value();
            (String::from(key), val.clone())
        });
        BTreeMap::from_iter(i)
    }

    pub fn new(columns: Vec<Column>) -> Self {
        Self { columns }
    }
}
