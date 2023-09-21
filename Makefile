VERSION_FILE := Cargo.toml

run:
	cargo run
build:
	cargo build
publish:
	sed -i -r "s/package.version=\"0\.0\.0\"/package.version=\"${VERSION}\"/g" "$(VERSION_FILE)" \
	  && sed -i -r "s/0\.0\.0/${VERSION}/g" "$(VERSION_FILE)" \
	  && cargo publish --allow-dirty --dry-run \

