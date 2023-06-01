export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1
EXPORT = export RUSTPATH=$(PWD)



migration:
	$(EXPORT) && sqlx migrate add -r ${title}

upgrade:
	$(EXPORT) && sqlx migrate run --database-url postgresql://migo:abc123@localhost:5433/rust-todo

downgrade:
	$(EXPORT) && sqlx migrate revert --database-url postgresql://migo:abc123@localhost:5433/rust-todo


test:
	$(EXPORT) && cargo test -- --test-threads 1


checks:
	$(EXPORT) && cargo fmt
	$(EXPORT) && cargo clippy

prepare:
	$(EXPORT) && DATABASE_URL='postgres://migo:abc123@localhost:5433/rust-todo' cargo sqlx prepare
	