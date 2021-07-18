OUTPUTDIR=./www/public
PROJECT_NAME=bomber

website: client
	rustup target add wasm32-unknown-unknown

	# Release build
	# cargo build -p bomber --target wasm32-unknown-unknown --release
	# cp target/wasm32-unknown-unknown/release/$(PROJECT_NAME).wasm $(OUTPUTDIR)/$(PROJECT_NAME).wasm

	# Debug build
	cargo build -p bomber --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/$(PROJECT_NAME).wasm $(OUTPUTDIR)/$(PROJECT_NAME).wasm


	rm -rf $(OUTPUTDIR)/assets
	cp -r client/assets $(OUTPUTDIR)/assets
	cd www && yarn install
	touch ./www/pages/index.tsx  # Refresh the yarn dev server
