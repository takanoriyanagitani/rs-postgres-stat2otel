use std::collections::BTreeMap;

use crate::{
    col::Column,
    gauge::{Gauge, GaugeType},
    label::Label,
    multi::{Multi, RawMulti},
    query::CustomQuery,
    row::Row,
    single::{RawSingle, Single},
};

use opentelemetry::{
    metrics::{Meter, ObservableGauge},
    Context, KeyValue, Value,
};

pub struct Metrics {
    integer: BTreeMap<String, ObservableGauge<i64>>,
    float: BTreeMap<String, ObservableGauge<f64>>,
}

impl Metrics {
    fn observe_integer(&self, c: &Context, key: &str, val: i64, kv: &[KeyValue]) {
        match self.integer.get(key) {
            None => {}
            Some(og) => og.observe(c, val, kv),
        }
    }

    fn observe_float(&self, c: &Context, key: &str, val: f64, kv: &[KeyValue]) {
        match self.float.get(key) {
            None => {}
            Some(og) => og.observe(c, val, kv),
        }
    }

    pub fn observe_single(&self, c: &Context, row: &Row, labels: &[Label]) {
        let m: BTreeMap<String, Value> = row.to_map();
        let attrs: Vec<KeyValue> = Label::to_attrs(labels, &m);

        let cols: &[Column] = row.as_columns();
        for col in cols {
            let key: &str = col.as_name();
            let val: &Value = col.as_value();
            match *val {
                Value::I64(i) => self.observe_integer(c, key, i, &attrs),
                Value::F64(f) => self.observe_float(c, key, f, &attrs),
                _ => {}
            }
        }
    }

    pub fn observe(&self, c: &Context, rows: &[Row], labels: &[Label]) {
        for row in rows {
            self.observe_single(c, row, labels)
        }
    }

    pub fn new(meter: &Meter, gs: &[Gauge]) -> Self {
        let mut integer: BTreeMap<String, ObservableGauge<i64>> = BTreeMap::new();
        let mut float: BTreeMap<String, ObservableGauge<f64>> = BTreeMap::new();
        for g in gs {
            let name: &str = g.as_name();
            let typ: GaugeType = g.as_type();
            match typ {
                GaugeType::Integer => {
                    let gi = g.to_integer_gauge(meter);
                    integer.insert(name.into(), gi);
                }
                GaugeType::Float => {
                    let gf = g.to_float_gauge(meter);
                    float.insert(name.into(), gf);
                }
            }
        }
        Metrics { integer, float }
    }
}

pub struct MetricsCollection {
    single: Vec<(Single, Metrics)>,
    multi: Vec<(Multi, Metrics)>,
}

impl MetricsCollection {
    fn new_sm(meter: &Meter, vs: Vec<Single>, vm: Vec<Multi>) -> Self {
        let single: Vec<(Single, Metrics)> = vs
            .into_iter()
            .map(|s: Single| {
                let m = Metrics::new(meter, s.as_gauge());
                (s, m)
            })
            .collect();
        let multi: Vec<(Multi, Metrics)> = vm
            .into_iter()
            .map(|multi: Multi| {
                let m = Metrics::new(meter, multi.as_gauge());
                (multi, m)
            })
            .collect();
        Self { single, multi }
    }

    pub fn new(meter: &Meter, mut q: CustomQuery) -> Self {
        let rs: Vec<RawSingle> = q.take_single().unwrap_or_default();
        let rm: Vec<RawMulti> = q.take_multi().unwrap_or_default();

        let vs: Vec<Single> = rs
            .into_iter()
            .flat_map(|r: RawSingle| Single::try_from(r).ok())
            .collect();

        let vm: Vec<Multi> = rm
            .into_iter()
            .flat_map(|r: RawMulti| Multi::try_from(r).ok())
            .collect();

        Self::new_sm(meter, vs, vm)
    }

    fn observe_single<D, G>(&self, data_source: &mut D, getter: &mut G, c: &Context)
    where
        G: FnMut(&mut D, &Single) -> Option<Row>,
    {
        for pair in &self.single {
            let (s, m) = pair;
            match getter(data_source, s) {
                None => {}
                Some(row) => m.observe_single(c, &row, s.as_label()),
            }
        }
    }

    fn observe_multi<D, G>(&self, data_source: &mut D, getter: &mut G, c: &Context)
    where
        G: FnMut(&mut D, &Multi) -> Vec<Row>,
    {
        for pair in &self.multi {
            let (multi, m) = pair;
            let v: Vec<Row> = getter(data_source, multi);
            let l: &[Label] = multi.as_label();
            m.observe(c, &v, l);
        }
    }

    pub fn observe<D, M, S>(
        &self,
        data_source: &mut D,
        get_single: &mut S,
        get_multi: &mut M,
        c: &Context,
    ) where
        S: FnMut(&mut D, &Single) -> Option<Row>,
        M: FnMut(&mut D, &Multi) -> Vec<Row>,
    {
        self.observe_single(data_source, get_single, c);
        self.observe_multi(data_source, get_multi, c);
    }
}
