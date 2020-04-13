build:
	wasm-pack build --scope wass80

deploy:
	(cd pkg; npm publish --access=public)