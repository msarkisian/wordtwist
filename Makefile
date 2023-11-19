COOKIE_KEY := server/web/cookie_key
DB := server/web/sql.db3
MIGRATIONS_DIR = server/web/migrations
SQL_FILES = $(MIGRATIONS_DIR)/1_setup.sql \
						$(MIGRATIONS_DIR)/2_daily.sql \
						$(MIGRATIONS_DIR)/3_user.sql \
						$(MIGRATIONS_DIR)/4_scores.sql \
						$(MIGRATIONS_DIR)/5_score_metadata.sql \
						$(MIGRATIONS_DIR)/6_game_size.sql \
						$(MIGRATIONS_DIR)/7_score_unique_idx.sql \

build_server: build_client $(COOKIE_KEY) $(DB)
	cargo build --release --manifest-path server/Cargo.toml
build_client: $(wildcard client/**/*)
	yarn --cwd client/ build


$(COOKIE_KEY):
			openssl rand 64 > server/web/cookie_key

dev: $(COOKIE_KEY) $(DB)
	yarn --cwd client/ run dev & cargo run --manifest-path server/Cargo.toml

gen_db: $(DB)

$(DB):
		$(foreach sqlfile,$(SQL_FILES),cat $(sqlfile) | sqlite3 $(DB);)

.PHONY: clean
clean:
		rm -f $(DB)