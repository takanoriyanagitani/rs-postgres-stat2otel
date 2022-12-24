use crate::{evt::Event, multi::RawMulti, single::RawSingle};

/// Metrics config collection(Single/Multi).
#[derive(serde::Deserialize)]
pub struct CustomQuery {
    multi: Option<Vec<RawMulti>>,
    single: Option<Vec<RawSingle>>,
}

impl CustomQuery {
    /// Takes `RawSingle` collection.
    pub fn take_single(&mut self) -> Option<Vec<RawSingle>> {
        self.single.take()
    }

    /// Takes `RawMulti` collection.
    pub fn take_multi(&mut self) -> Option<Vec<RawMulti>> {
        self.multi.take()
    }
}

/// Parses custom query config bytes to create `CustomQuery` using toml parser.
#[cfg(feature = "config_toml")]
pub fn from_toml_slice(b: &[u8]) -> Result<CustomQuery, Event> {
    toml::de::from_slice(b)
        .map_err(|e| Event::InvalidConfig(format!("Unable to parse toml bytes: {}", e)))
}

/// Parses custom query config bytes to create `CustomQuery` using default parser.
#[cfg(feature = "config_toml")]
pub fn from_slice_default(b: &[u8]) -> Result<CustomQuery, Event> {
    from_toml_slice(b)
}

#[cfg(test)]
mod test_query {

    #[cfg(feature = "config_toml")]
    mod test_toml {
        use crate::{
            gauge::{Gauge, GaugeType},
            label::Label,
            multi::{Multi, RawMulti},
            query::{from_toml_slice, CustomQuery},
            single::{RawSingle, Single},
        };

        #[test]
        fn test_empty() {
            let c: CustomQuery = from_toml_slice(&[]).unwrap();
            assert!(c.multi.is_none());
            assert!(c.single.is_none());
        }

        #[test]
        fn test_single() {
            let single: &str = r#"
                [[single]]
                
                query = '''
                    SELECT
                        wal_records::BIGINT,
                        wal_fpi::BIGINT,
                        wal_bytes::BIGINT,
                        wal_buffers_full::BIGINT,
                        wal_write::BIGINT,
                        wal_sync::BIGINT,
                        wal_write_time::FLOAT8,
                        wal_sync_time::FLOAT8,
                        EXTRACT(EPOCH FROM stats_reset)::FLOAT8 AS stats_reset
                    FROM pg_stat_wal
                    LIMIT 1
                '''
                
                [[single.gauge]]
                name = "wal_records"
                gauge_type = "i64"
                description = "Total number of WAL records generated."
                unit = ""
                
                [[single.gauge]]
                name = "wal_fpi"
                gauge_type = "i64"
                description = "Total number of WAL full page images generated."
                unit = ""
            "#;

            let mut c: CustomQuery = from_toml_slice(single.as_bytes()).unwrap();
            assert!(c.single.is_some());
            assert!(c.multi.is_none());

            let o: Option<Vec<RawSingle>> = c.take_single();
            assert!(o.is_some());
            let vs: Vec<RawSingle> = o.unwrap();
            assert_eq!(vs.len(), 1);
            let rs: RawSingle = vs.into_iter().next().unwrap();

            let mut s: Single = Single::try_from(rs).unwrap();

            let ovl: Option<Vec<Label>> = s.take_label();
            assert!(ovl.is_none());

            assert!(0 < s.as_query().len());

            let g: &[Gauge] = s.as_gauge();
            assert_eq!(g.len(), 2);
            let wal_records: &Gauge = &g[0];
            let wal_fpi: &Gauge = &g[1];

            assert_eq!(wal_records.as_name(), "wal_records");
            assert_eq!(wal_records.as_type(), GaugeType::Integer);
            assert!(0 < wal_records.as_desc().len());
            assert_eq!(wal_records.as_unit(), "");

            assert_eq!(wal_fpi.as_name(), "wal_fpi");
            assert_eq!(wal_fpi.as_type(), GaugeType::Integer);
            assert!(0 < wal_fpi.as_desc().len());
            assert_eq!(wal_fpi.as_unit(), "");
        }

        #[test]
        fn test_multi() {
            let multi: &str = r#"
                [[multi]]
                
                query = '''
                    SELECT
                        datname::TEXT,
                        application_name::TEXT,
                        EXTRACT(SECOND FROM (CLOCK_TIMESTAMP() - query_start))::FLOAT8 AS elapsed,
                        wait_event_type::TEXT,
                        wait_event::TEXT,
                        state::TEXT,
                        backend_type::TEXT
                    FROM pg_stat_activity
                '''
                
                [[multi.label]]
                name = "datname"
                description = "Name of this database."
                
                [[multi.label]]
                name = "application_name"
                description = "Name of the application that is connected to this backend."
                
                [[multi.label]]
                name = "wait_event_type"
                description = "The type of event for which the backend is waiting."
                
                [[multi.gauge]]
                name = "elapsed"
                gauge_type = "f64"
                description = "Time elapsed for this query."
                unit = "seconds"
            "#;
            let mut c: CustomQuery = from_toml_slice(multi.as_bytes()).unwrap();
            assert!(c.take_single().is_none());

            let ovrm: Option<Vec<RawMulti>> = c.take_multi();
            assert!(ovrm.is_some());

            let vrm: Vec<RawMulti> = ovrm.unwrap();
            assert_eq!(vrm.len(), 1);

            let rm: RawMulti = vrm.into_iter().next().unwrap();

            let m: Multi = Multi::try_from(rm).unwrap();

            assert!(0 < m.as_query().len());
            let sl: &[Label] = m.as_label();

            let datname: &Label = &sl[0];
            let application_name: &Label = &sl[1];
            let wait_event_type: &Label = &sl[2];

            assert_eq!(datname.as_name(), "datname");
            assert_eq!(application_name.as_name(), "application_name");
            assert_eq!(wait_event_type.as_name(), "wait_event_type");

            assert!(0 < datname.as_desc().len());
            assert!(0 < application_name.as_desc().len());
            assert!(0 < wait_event_type.as_desc().len());

            let sg: &[Gauge] = m.as_gauge();
            assert_eq!(sg.len(), 1);
            let g: &Gauge = &sg[0];

            assert_eq!(g.as_name(), "elapsed");
            assert_eq!(g.as_type(), GaugeType::Float);
            assert_eq!(g.as_unit(), "seconds");
        }
    }
}
