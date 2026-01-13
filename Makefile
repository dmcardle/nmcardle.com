.PHONY: build
build:
	bazelisk build //:release

.PHONY: format
format:
	nix-shell -p prettier --command "prettier --write **/*.js"

.PHONY: debug
debug:
	./release.sh --destructive
	nix-shell --command "firebase emulators:start"
