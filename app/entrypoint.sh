#!/bin/bash

cd /usr/local/bin/app

./diesel migration run --config-file=diesel.toml --database-url "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:5432/${DB_NAME}?sslmode=disable"

./app
