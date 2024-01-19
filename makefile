build: 
	cargo build --release

test: 
	cargo run

prod: 
	cargo run --release

lint: 
	cargo fmt

update:
	cargo update