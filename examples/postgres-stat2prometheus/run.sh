#!/bin/sh

export PGHOST=localhost
export PGPORT=5432
export PGUSER=postgres
export PGPASSWORD=postgres
export PGDATABASE=postgres

export OTEL_SERVICE_NAME=test-pg2otel

names(){
    echo sample.d/pg12.pg_stat_database.toml
    #echo sample.d/pg_stat_database.toml
    echo sample.d/pg_stat_activity.toml
}

main(){
    cat $( names ) |
        ./target/release/postgres-stat2prometheus
}

main
