//! Metrics label

use std::collections::BTreeMap;

use crate::evt::Event;

use opentelemetry::{KeyValue, Value};

/// Metrics label config info.
#[derive(serde::Deserialize)]
pub struct Label {
    name: String,
    description: String,
}

impl Label {
    /// Gets the name.
    pub fn as_name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the description.
    pub fn as_desc(&self) -> &str {
        self.description.as_str()
    }

    /// Converts to `KeyValue` if the map contains the label value.
    pub fn to_kv(&self, m: &BTreeMap<String, Value>) -> Result<KeyValue, Event> {
        let v: &Value = m.get(self.name.as_str()).ok_or_else(|| {
            Event::LabelValueNotFound(format!("Value for label missing: {}", self.name))
        })?;
        Ok(KeyValue::new(self.name.clone(), v.clone()))
    }

    /// Converts labels to `KeyValue`s.
    ///
    /// Labels without a value will be ignored.
    pub fn to_attrs(s: &[Self], m: &BTreeMap<String, Value>) -> Vec<KeyValue> {
        s.iter().flat_map(|l: &Self| l.to_kv(m).ok()).collect()
    }
}
