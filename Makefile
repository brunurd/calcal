help:
	@cat Makefile

run:
	cargo run $(filter-out $@,$(MAKECMDGOALS))

build:
	cargo build

release:
	cargo build --release

macos-release:
	cargo lipo --release

debug:
	@make build
	gdb target/debug/calcalcli
