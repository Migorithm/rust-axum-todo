export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1
EXPORT = export RUSTPATH=$(PWD)



migration:
	$(EXPORT) && sqlx migrate add -r ${title}

upgrade:
	$(EXPORT) && sqlx migrate run --database-url postgresql://localhost:5432/rustweb

downgrade:
	$(EXPORT) && sqlx migrate revert --database-url postgresql://localhost:5432/rustweb


test:
	$(EXPORT) && cargo test -- --test-threads 1


checks:
	$(EXPORT) && cargo fmt
	$(EXPORT) && cargo clippy
