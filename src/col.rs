use opentelemetry::Value;

/// A metrics value(with name) container.
pub struct Column {
    name: String,
    value: Value,
}

impl Column {
    /// Gets a name.
    pub fn as_name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets a value.
    pub fn as_value(&self) -> &Value {
        &self.value
    }

    /// Creates a new name/integer container.
    pub fn new_integer(name: String, value: i64) -> Self {
        Self {
            name,
            value: Value::from(value),
        }
    }

    /// Creates a new name/float container.
    pub fn new_float(name: String, value: f64) -> Self {
        Self {
            name,
            value: Value::from(value),
        }
    }

    /// Creates a new name/string container.
    pub fn new_string(name: String, value: String) -> Self {
        Self {
            name,
            value: Value::from(value),
        }
    }
}
