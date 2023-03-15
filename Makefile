COOKIE_KEY := server/web/cookie_key

build_server: build_client $(COOKIE_KEY)
	cargo build --release --manifest-path server/Cargo.toml
build_client: $(wildcard client/**/*)
	yarn --cwd client/ build


$(COOKIE_KEY):
			openssl rand 64 > server/web/cookie_key

dev: $(COOKIE_KEY)
	yarn --cwd client/ run dev & cargo run --manifest-path server/Cargo.toml