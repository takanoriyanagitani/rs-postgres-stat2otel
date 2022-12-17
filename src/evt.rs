//! List of events

/// List of events(errors).
#[derive(Debug)]
pub enum Event {
    InvalidConfig(String),
    InvalidGauge(String),
    LabelValueNotFound(String),
}
