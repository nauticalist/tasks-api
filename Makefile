DATABASE_URL := postgres://dbuser:passwd123@localhost/app?sslmode=disable
MAX_CONNECTIONS := 5

export DATABASE_URL
export MAX_CONNECTIONS

.PHONY: setenv_db

setenv_db:
	fish -C "set -Ux DATABASE_URL ${DATABASE_URL}"
	fish -C "set -Ux MAX_CONNECTIONS ${MAX_CONNECTIONS}"

run:
	cargo run -p api