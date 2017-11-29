setup:
	rustup update 
	rustup target add wasm32-unknown-unknown --toolchain nightly

build:
	cd app && cargo build 
	cp app/target/wasm32-unknown-unknown/debug/app.wasm site/.

run:
	cd site && python -m SimpleHTTPServer
