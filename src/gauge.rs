//! A Gauge builder

use crate::evt::Event;

use opentelemetry::metrics::{Meter, ObservableGauge, Unit};

/// List of gauge number type.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum GaugeType {
    Float,
    Integer,
}

impl TryFrom<&str> for GaugeType {
    type Error = Event;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "f64" => Ok(Self::Float),
            "i64" => Ok(Self::Integer),
            "i32" => Ok(Self::Integer),
            _ => Err(Event::InvalidGauge(format!("Unknown gauge type: {}", s))),
        }
    }
}

/// Serialized `Gauge`.
#[derive(serde::Deserialize)]
pub struct RawGauge {
    name: String,
    gauge_type: String,
    description: String,
    unit: String,
}

/// A gauge config.
pub struct Gauge {
    /// The name of the gauge.
    name: String,

    /// The type of this gauge.
    typ: GaugeType,

    description: String,

    /// The unit of this gauge(can be empty).
    unit: String,
}

impl Gauge {
    /// Gets the name of this gauge.
    pub fn as_name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the type of this gauge.
    pub fn as_type(&self) -> GaugeType {
        self.typ
    }

    /// Gets the description of this gauge.
    pub fn as_desc(&self) -> &str {
        self.description.as_str()
    }

    /// Gets the unit string of this gauge.
    pub fn as_unit(&self) -> &str {
        self.unit.as_str()
    }

    /// Gets the unit of this gauge if exists.
    pub fn to_unit(&self) -> Option<Unit> {
        let empty: bool = self.unit.is_empty();
        (!empty).then(|| Unit::new(self.unit.clone()))
    }

    /// Creates `ObservableGauge`(i64).
    pub fn to_integer_gauge(&self, m: &Meter) -> ObservableGauge<i64> {
        let builder = m
            .i64_observable_gauge(self.as_name())
            .with_description(self.as_desc());
        match self.to_unit() {
            None => builder.init(),
            Some(u) => builder.with_unit(u).init(),
        }
    }

    /// Creates `ObservableGauge`(f64).
    pub fn to_float_gauge(&self, m: &Meter) -> ObservableGauge<f64> {
        let builder = m
            .f64_observable_gauge(self.as_name())
            .with_description(self.as_desc());
        match self.to_unit() {
            None => builder.init(),
            Some(u) => builder.with_unit(u).init(),
        }
    }
}

impl TryFrom<RawGauge> for Gauge {
    type Error = Event;
    fn try_from(r: RawGauge) -> Result<Self, Self::Error> {
        let typ: GaugeType = GaugeType::try_from(r.gauge_type.as_str())?;

        let name = r.name;
        let description = r.description;
        let unit = r.unit;

        Ok(Self {
            name,
            typ,
            description,
            unit,
        })
    }
}
