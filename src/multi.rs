use crate::{
    evt::Event,
    gauge::{Gauge, RawGauge},
    label::Label,
};

/// Multi gets gauge items from many rows(Label required).
pub struct Multi {
    query: String,
    label: Vec<Label>,
    gauge: Vec<Gauge>,
}

impl Multi {
    /// Gets query string to get metrics rows.
    pub fn as_query(&self) -> &str {
        self.query.as_str()
    }

    /// Gets labels.
    pub fn as_label(&self) -> &[Label] {
        &self.label
    }

    /// Gets gauges.
    pub fn as_gauge(&self) -> &[Gauge] {
        &self.gauge
    }
}

impl TryFrom<RawMulti> for Multi {
    type Error = Event;
    fn try_from(r: RawMulti) -> Result<Self, Self::Error> {
        let query: String = r.query;
        let label: Vec<_> = r.label;

        let gauge: Vec<Gauge> = r.gauge.into_iter().try_fold(vec![], |mut v, r| {
            let g: Gauge = Gauge::try_from(r)?;
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

/// Raw(serialized) Multi.
#[derive(serde::Deserialize)]
pub struct RawMulti {
    query: String,
    label: Vec<Label>,
    gauge: Vec<RawGauge>,
}
