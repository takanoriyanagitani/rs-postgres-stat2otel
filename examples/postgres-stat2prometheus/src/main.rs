use std::env;
use std::io::{stdin, stdout, Read, Write};

use opentelemetry::sdk::{
    export::metrics::aggregation,
    metrics::{controllers, processors, selectors},
};
use opentelemetry::{global, metrics::Meter, Context};

use postgres::{types::Type, Client, Config, NoTls};
use prometheus::{Encoder, TextEncoder};

use rs_postgres_stat2otel::{
    col::Column,
    metrics::MetricsCollection,
    multi::Multi,
    query::{from_slice_default, CustomQuery},
    row::Row,
    single::Single,
};

fn pg_new_client() -> Result<Client, String> {
    Config::new()
        .host(env::var("PGHOST").unwrap_or_default().as_str())
        .user(env::var("PGUSER").unwrap_or_default().as_str())
        .password(env::var("PGPASSWORD").unwrap_or_default().as_str())
        .connect(NoTls)
        .map_err(|e| format!("Unable to connect: {}", e))
}

fn pgcol2col(r: &postgres::Row, c: &postgres::Column) -> Option<Column> {
    let name: &str = c.name();

    let igen = |i: i64| Column::new_integer(name.into(), i);
    let fgen = |f: f64| Column::new_float(name.into(), f);
    let sgen = |s: &str| Column::new_string(name.into(), s.into());

    match *c.type_() {
        Type::INT8 => r.try_get(name).ok().map(igen),
        Type::INT4 => r.try_get(name).ok().map(|i: i32| i.into()).map(igen),
        Type::INT2 => r.try_get(name).ok().map(|i: i16| i.into()).map(igen),
        Type::FLOAT8 => r.try_get(name).ok().map(fgen),
        Type::FLOAT4 => r.try_get(name).ok().map(|f: f32| f.into()).map(fgen),
        Type::TEXT => r.try_get(name).ok().map(sgen),
        _ => {
            println!("unknown type(name={}): {:#?}", name, c);
            None
        }
    }
}

fn get_single(c: &mut Client, s: &Single) -> Option<Row> {
    let query: &str = s.as_query();
    let o: Option<postgres::Row> = c.query_opt(query, &[]).ok().flatten();
    match o {
        None => {
            eprintln!("Empty row.");
            None
        }
        Some(pr) => {
            let pc: &[postgres::Column] = pr.columns();
            let cols: Vec<Column> = pc
                .iter()
                .flat_map(|c: &postgres::Column| pgcol2col(&pr, c))
                .collect();
            Some(Row::new(cols))
        }
    }
}

fn get_multi(c: &mut Client, s: &Multi) -> Vec<Row> {
    let query: &str = s.as_query();
    let v: Vec<postgres::Row> = c.query(query, &[]).ok().unwrap_or_default();
    v.is_empty().then(|| eprintln!("Empty rows"));
    v.into_iter()
        .map(|pr: postgres::Row| {
            let pc: &[postgres::Column] = pr.columns();
            let cols: Vec<Column> = pc
                .iter()
                .flat_map(|c: &postgres::Column| pgcol2col(&pr, c))
                .collect();
            Row::new(cols)
        })
        .collect()
}

fn observe(m: &MetricsCollection, ctx: &Context, client: &mut Client) {
    m.observe(client, &mut get_single, &mut get_multi, ctx)
}

fn new_metrics_collection(source: &[u8]) -> Result<MetricsCollection, String> {
    let q: CustomQuery =
        from_slice_default(source).map_err(|e| format!("Invalid config: {:#?}", e))?;
    let m: Meter = global::meter("test.postgres-stat2otel");
    Ok(MetricsCollection::new(&m, q))
}

fn init_prometheus() {
    let controller = controllers::basic(
        processors::factory(
            selectors::simple::histogram([-0.5, 1.0]),
            aggregation::cumulative_temporality_selector(),
        )
        .with_memory(true),
    )
    .build();

    opentelemetry_prometheus::exporter(controller)
        .with_registry(prometheus::default_registry().clone())
        .init();
}

fn sub() -> Result<(), String> {
    init_prometheus();

    let mut buf: Vec<u8> = Vec::new();
    let mut il = stdin().lock();
    il.read_to_end(&mut buf)
        .map_err(|e| format!("Unable to read config: {}", e))?;
    let mc: MetricsCollection = new_metrics_collection(&buf)?;
    let ctx: Context = Context::current();
    let mut client: Client = pg_new_client()?;
    observe(&mc, &ctx, &mut client);

    let te: TextEncoder = TextEncoder::new();
    let metrics_items: Vec<_> = prometheus::default_registry().gather();
    let mut metrics_out: Vec<u8> = Vec::new();
    te.encode(&metrics_items, &mut metrics_out)
        .map_err(|e| format!("Unable to serialize metrics: {}", e))?;

    let mut ol = stdout().lock();
    ol.write_all(&metrics_out)
        .map_err(|e| format!("Unable to write metrics: {}", e))?;

    Ok(())
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}
