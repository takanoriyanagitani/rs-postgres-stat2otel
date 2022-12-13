use crate::evt::Event;

use opentelemetry::metrics::{Meter, ObservableGauge, Unit};

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

#[derive(serde::Deserialize)]
pub struct RawGauge {
    name: String,
    gauge_type: String,
    description: String,
    unit: String,
}

pub struct Gauge {
    name: String,
    typ: GaugeType,
    description: String,
    unit: String,
}

impl Gauge {
    pub fn as_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn as_type(&self) -> GaugeType {
        self.typ
    }

    pub fn as_desc(&self) -> &str {
        self.description.as_str()
    }

    pub fn as_unit(&self) -> &str {
        self.unit.as_str()
    }

    pub fn to_unit(&self) -> Option<Unit> {
        let empty: bool = self.unit.is_empty();
        (!empty).then(|| Unit::new(self.unit.clone()))
    }

    pub fn to_integer_gauge(&self, m: &Meter) -> ObservableGauge<i64> {
        let builder = m
            .i64_observable_gauge(self.as_name())
            .with_description(self.as_desc());
        match self.to_unit() {
            None => builder.init(),
            Some(u) => builder.with_unit(u).init(),
        }
    }

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
