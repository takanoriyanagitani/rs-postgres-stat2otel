use crate::{
    evt::Event,
    gauge::{Gauge, RawGauge},
    label::Label,
};

pub struct Multi {
    query: String,
    label: Vec<Label>,
    gauge: Vec<Gauge>,
}

impl Multi {
    pub fn as_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn as_label(&self) -> &[Label] {
        &self.label
    }

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

#[derive(serde::Deserialize)]
pub struct RawMulti {
    query: String,
    label: Vec<Label>,
    gauge: Vec<RawGauge>,
}
