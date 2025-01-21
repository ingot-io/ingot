migration:
	diesel migration run

migration-revert:
	diesel migration revert

generate-migration:
	diesel migration generate $(name)

build:
	cargo build

run:
	cargo run

test:
	cargo test

clean:
	cargo clean

grpc-build:
	protoc --rust_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_rust_plugin` proto/*.proto

grpc-run:
	cargo run --bin grpc_server

grpc-clean:
	rm -rf src/*.rs