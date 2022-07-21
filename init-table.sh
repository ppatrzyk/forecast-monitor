#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE TABLE forecasts(
        id SERIAL PRIMARY KEY ,
        source VARCHAR(255),
        forecast_time VARCHAR(255),
        weather_time VARCHAR(255),
        temperature REAL,
        precipitation REAL
    )
EOSQL