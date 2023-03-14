build_server: build_client cookie_key
	cargo build --release --manifest-path server/Cargo.toml
build_client: $(wildcard client/**/*)
	yarn --cwd client/ build

cookie_key:
	openssl rand 64 > server/web/cookie_key

dev: 
	yarn --cwd client/ run dev & cargo run --manifest-path server/Cargo.toml