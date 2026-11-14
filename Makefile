.PHONY: check rust-check python-check db-up db-down

check: rust-check python-check

rust-check:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace

python-check:
	python -m pytest
	python -m ruff check .

db-up:
	docker compose up -d postgres

db-down:
	docker compose down
