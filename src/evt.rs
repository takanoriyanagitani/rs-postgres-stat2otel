#[derive(Debug)]
pub enum Event {
    InvalidConfig(String),
    InvalidGauge(String),
    LabelValueNotFound(String),
}
