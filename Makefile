include DEVOPS/dev/.env

DIESEL_CMD := ~/.cargo/bin/diesel
SCHEMA_FILE := app/src/schema.rs

dev live: docker-postgres
	bacon ./app run -- ../DEVOPS/dev/.env

migrate:
	${DIESEL_CMD} migration run --config-file=diesel.toml --database-url ${DATABASE_URL}

migrate-redo:
	${DIESEL_CMD} migration redo --config-file=diesel.toml --database-url ${DATABASE_URL}

diesel:
	@echo "Fetching schemas from database and writing to $(SCHEMA_FILE)..."
	# 1. Clear the target file first using '>'
	@echo "" > $(SCHEMA_FILE)

	# 2. Run the complex shell logic (all on one logical line via '\')
	@SCHEMAS=$$(PGPASSWORD='${DB_PASSWORD}' psql -h 127.0.0.1 -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -Atc "SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast')"); \
	echo "Schemas found: $$SCHEMAS"; \
	for SCHEMA in $$SCHEMAS; do \
		echo "//--- $$SCHEMA ---" >> $(SCHEMA_FILE); \
		${DIESEL_CMD} print-schema --database-url=$(DATABASE_URL) -s=$$SCHEMA >> $(SCHEMA_FILE); \
	done

	@echo "Schema generation complete."

docker-postgres:
	systemctl start docker
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=DEVOPS/dev/.env up -d rust_starter_db --remove-orphans

docker-postgres-stop:
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=DEVOPS/dev/.env down rust_starter_db

rm-docker-postgres:
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=DEVOPS/dev/.env down rust_starter_db -v
