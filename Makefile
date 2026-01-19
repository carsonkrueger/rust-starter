include app/.env

DIESEL_CMD := ~/.cargo/bin/diesel
APP_SCHEMA_FILE := libs/schemas/lib.rs
APP_DIESEL_TOML := app/diesel.toml
APP_ENV := DEVOPS/dev/.env
DB_URL := "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}?sslmode=disable"

app: docker-postgres
	cargo watch -q -C ./app -x 'run -- .env' -d 0.2

tw:
	npx @tailwindcss/cli -i app/app.css -o app/public/css/index.css --watch

tw-once:
	npx @tailwindcss/cli -i app/app.css -o app/public/css/index.css

migrate-create:
	@read -p "Enter migration name: " name; \
	${DIESEL_CMD} migration generate --config-file=${APP_DIESEL_TOML} --database-url ${DB_URL} $$name

migrate:
	${DIESEL_CMD} migration run --config-file=${APP_DIESEL_TOML} --database-url ${DB_URL}

migrate-redo:
	${DIESEL_CMD} migration redo --config-file=${APP_DIESEL_TOML} --database-url ${DB_URL}

migrate-down:
	${DIESEL_CMD} migration revert --config-file=${APP_DIESEL_TOML} --database-url ${DB_URL}

diesel:
	@echo "Fetching schemas from database and writing to $(SCHEMA_FILE)..."
	# Clear the target file
	@echo "" > $(APP_SCHEMA_FILE)

	# Find all schemas
	@SCHEMAS=$$(PGPASSWORD='${DB_PASSWORD}' psql -h 127.0.0.1 -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -Atc "SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast')"); \
	echo "Schemas found: $$SCHEMAS"; \
	for SCHEMA in $$SCHEMAS; do \
		echo "//--- $$SCHEMA ---" >> $(APP_SCHEMA_FILE); \
		${DIESEL_CMD} print-schema --database-url=$(DB_URL) -s=$$SCHEMA >> $(APP_SCHEMA_FILE); \
	done

	@echo "Schema generation complete."

docker-postgres:
	@systemctl is-active --quiet docker || sudo systemctl start docker
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=${APP_ENV} up -d app_db --remove-orphans

docker-postgres-stop:
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=${APP_ENV} down app_db

rm-docker-postgres:
	docker compose -f DEVOPS/dev/docker-compose.yml --env-file=${APP_ENV} down app_db -v

docker: docker-down tw-once
	docker compose -f ./DEVOPS/dev/docker-compose.yml --env-file ./DEVOPS/dev/.env up -d --build

docker-down:
	docker compose -f ./DEVOPS/dev/docker-compose.yml --env-file ./DEVOPS/dev/.env down
