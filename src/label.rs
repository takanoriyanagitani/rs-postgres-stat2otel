use std::collections::BTreeMap;

use crate::evt::Event;

use opentelemetry::{KeyValue, Value};

#[derive(serde::Deserialize)]
pub struct Label {
    name: String,
    description: String,
}

impl Label {
    pub fn as_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn as_desc(&self) -> &str {
        self.description.as_str()
    }

    pub fn to_kv(&self, m: &BTreeMap<String, Value>) -> Result<KeyValue, Event> {
        let v: &Value = m.get(self.name.as_str()).ok_or_else(|| {
            Event::LabelValueNotFound(format!("Value for label missing: {}", self.name))
        })?;
        Ok(KeyValue::new(self.name.clone(), v.clone()))
    }

    pub fn to_attrs(s: &[Self], m: &BTreeMap<String, Value>) -> Vec<KeyValue> {
        s.iter().flat_map(|l: &Self| l.to_kv(m).ok()).collect()
    }
}
