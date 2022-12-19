//! Single set of metrics items / single request
//!
//! A query must return one row.

use crate::{
    evt::Event,
    gauge::{Gauge, RawGauge},
    label::Label,
};

/// Single gets gauge items from single row.
pub struct Single {
    query: String,
    label: Option<Vec<Label>>,
    gauge: Vec<Gauge>,
}

impl TryFrom<RawSingle> for Single {
    type Error = Event;
    fn try_from(r: RawSingle) -> Result<Self, Self::Error> {
        let query: String = r.query;
        let label: Option<_> = r.label;

        let gauge: Vec<Gauge> = r.gauge.into_iter().try_fold(vec![], |mut v, raw| {
            let g: Gauge = Gauge::try_from(raw)?;
            v.push(g);
            Ok(v)
        })?;
        Ok(Self {
            query,
            label,
            gauge,
        })
    }
}

/// Raw(serialized) Single.
#[derive(serde::Deserialize)]
pub struct RawSingle {
    query: String,
    label: Option<Vec<Label>>,
    gauge: Vec<RawGauge>,
}

impl Single {
    /// Gets the query string to get a metrics row.
    pub fn as_query(&self) -> &str {
        self.query.as_str()
    }

    /// Takes all labels if exists.
    pub fn take_label(&mut self) -> Option<Vec<Label>> {
        self.label.take()
    }

    /// Gets all labels(can be empty).
    pub fn as_label(&self) -> &[Label] {
        match &self.label {
            None => &[],
            Some(l) => l,
        }
    }

    /// Gets all gauge items.
    pub fn as_gauge(&self) -> &[Gauge] {
        &self.gauge
    }
}
