//! A row which contains columns.

use std::collections::BTreeMap;

use crate::col::Column;

use opentelemetry::Value;

/// Contains many columns.
pub struct Row {
    columns: Vec<Column>,
}

impl Row {
    /// Gets columns.
    pub fn as_columns(&self) -> &[Column] {
        &self.columns
    }

    /// Converts to a map.
    ///
    /// - Key: The name of the column.
    /// - Val: The value of the column.
    pub fn to_map(&self) -> BTreeMap<String, Value> {
        let i = self.as_columns().iter().map(|c: &Column| {
            let key: &str = c.as_name();
            let val: &Value = c.as_value();
            (String::from(key), val.clone())
        });
        BTreeMap::from_iter(i)
    }

    /// Creates new row from columns.
    pub fn new(columns: Vec<Column>) -> Self {
        Self { columns }
    }
}
