SQLITE_DB := m2.sqlite3

.PHONY: e2e setup build run clean

e2e: clean setup build run

setup:
	@rm -f $(SQLITE_DB)
	@touch $(SQLITE_DB)
	@chmod 666 $(SQLITE_DB)

build:
	@docker compose build

run:
	@docker compose up --abort-on-container-exit --exit-code-from karate

clean:
	@docker compose down -v || true

