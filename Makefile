build_server: build_client
	cargo build --release --manifest-path server/Cargo.toml
build_client: $(wildcard client/**/*)
	yarn --cwd client/ build

dev: 
	yarn --cwd client/ run dev & cargo run --manifest-path server/Cargo.toml