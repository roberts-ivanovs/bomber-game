OUTPUTDIR=./web

website: src
	rustup target add wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/bomber.wasm web/bomber.wasm
	cp -r assets web/assets
