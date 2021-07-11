OUTPUTDIR=./www/public
PROJECT_NAME=bomber

website: src
	rustup target add wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/$(PROJECT_NAME).wasm $(OUTPUTDIR)/$(PROJECT_NAME).wasm
	rm -r $(OUTPUTDIR)/assets
	cp -r assets $(OUTPUTDIR)/assets
	cd www && yarn install
	touch ./www/pages/index.tsx  # Refresh the yarn dev server
