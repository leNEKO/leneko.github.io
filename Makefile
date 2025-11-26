dev_shell:
	nix-shell

schemas:
	cargo run -- schema cv > data/cv.schema.json
	cargo run -- schema assets > data/assets.schema.json
	cargo run -- schema technos > data/technos.schema.json

index:
	cargo run -- build data/cv.yml | npx prettier@latest --parser html > index.html


watch: dev_shell
	xdg-open index.html
	cargo watch -w src -w data -s "make index"
