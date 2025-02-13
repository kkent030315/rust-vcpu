DEBUG ?= 0
RUST_BUILD_PROFILE := $(if $(filter 1,$(DEBUG)),debug,release)
RUST_BUILD_FLAG := $(if $(filter 1,$(DEBUG)),, --release)

test-compiler:
	@cargo test -p compiler --all $(RUST_BUILD_FLAG)

test-vm:
	@cargo test -p vm --all $(RUST_BUILD_FLAG)

test-all: test-compiler test-vm

docs:
	@cargo doc

dist-compiler:
	@cargo build -p compiler $(RUST_BUILD_FLAG)

dist-wasm:
	@cd www/wasm && wasm-pack build $(RUST_BUILD_FLAG) --target web

dist-all: dist-compiler

www-dev: dist-wasm
	@cd www && npm run dev

benchmark:
	@cargo bench -p benchmark

clean:
	@cargo clean
