include DEVOPS/dev/.env

DIESEL_CMD := ~/.cargo/bin/diesel
SCHEMA_FILE := libs/schemas/lib.rs

app live: docker-postgres
	cargo watch -q -C ./app -x 'run -- ../DEVOPS/dev/.env'

tw:
	npx @tailwindcss/cli -i app/app.css -o app/public/css/index.css --watch

migrate:
	${DIESEL_CMD} migration run --config-file=diesel.toml --database-url ${DATABASE_URL}

migrate-redo:
	${DIESEL_CMD} migration redo --config-file=diesel.toml --database-url ${DATABASE_URL}

diesel:
	@echo "Fetching schemas from database and writing to $(SCHEMA_FILE)..."
	# Clear the target file
	@echo "" > $(SCHEMA_FILE)

	# Find all schemas
	@SCHEMAS=$$(PGPASSWORD='${DB_PASSWORD}' psql -h 127.0.0.1 -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -Atc "SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast')"); \
	echo "Schemas found: $$SCHEMAS"; \
	for SCHEMA in $$SCHEMAS; do \
		echo "//--- $$SCHEMA ---" >> $(SCHEMA_FILE); \
		${DIESEL_CMD} print-schema --database-url=$(DATABASE_URL) -s=$$SCHEMA >> $(SCHEMA_FILE); \
	done

	@echo "Schema generation complete."

docker-postgres:
	@systemctl is-active --quiet docker || sudo systemctl start docker
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=DEVOPS/dev/.env up -d rust_starter_db --remove-orphans

docker-postgres-stop:
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=DEVOPS/dev/.env down rust_starter_db

rm-docker-postgres:
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=DEVOPS/dev/.env down rust_starter_db -v
