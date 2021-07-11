OUTPUTDIR=./www

website: src
	rustup target add wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/bomber.wasm $(OUTPUTDIR)/public/bomber.wasm
	rm -r $(OUTPUTDIR)/public/assets
	cp -r assets $(OUTPUTDIR)/public/assets
	cd www && yarn install
	touch www/pages/index.tsx
