use crate::{
    evt::Event,
    gauge::{Gauge, RawGauge},
    label::Label,
};

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

#[derive(serde::Deserialize)]
pub struct RawSingle {
    query: String,
    label: Option<Vec<Label>>,
    gauge: Vec<RawGauge>,
}

impl Single {
    pub fn as_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn take_label(&mut self) -> Option<Vec<Label>> {
        self.label.take()
    }

    pub fn as_label(&self) -> &[Label] {
        match &self.label {
            None => &[],
            Some(l) => l,
        }
    }

    pub fn as_gauge(&self) -> &[Gauge] {
        &self.gauge
    }
}
