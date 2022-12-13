use opentelemetry::Value;

pub struct Column {
    name: String,
    value: Value,
}

impl Column {
    pub fn as_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn as_value(&self) -> &Value {
        &self.value
    }

    pub fn new_integer(name: String, value: i64) -> Self {
        Self {
            name,
            value: Value::from(value),
        }
    }

    pub fn new_float(name: String, value: f64) -> Self {
        Self {
            name,
            value: Value::from(value),
        }
    }

    pub fn new_string(name: String, value: String) -> Self {
        Self {
            name,
            value: Value::from(value),
        }
    }
}
