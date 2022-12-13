#!/bin/sh

export PGHOST=localhost
export PGPORT=5432
export PGUSER=postgres
export PGPASSWORD=postgres
export PGDATABASE=postgres

export OTEL_SERVICE_NAME=test-pg2otel

cat \
  sample.d/pg12.pg_stat_database.toml \
  sample.d/pg_stat_activity.toml \
  | ./target/release/postgres-stat2prometheus
