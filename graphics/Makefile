build:
	mkdir -p ../view/public/resources
	cp -r resources ../view/public
	echo "*" > ../view/public/resources/.gitignore
	wasm-pack build --target web --out-dir ../view/pkg