# verify {{{
verify\:error: # Check error [alias: verify:err, error, err]
	@cargo check --all --verbose
.PHONY: verify\:error

verify\:err: verify\:error
.PHONY: verify\:err

error: verify\:error
.PHONY: error

err: verify\:error
.PHONY: err

verify\:format: # Show format diff [alias: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

verify\:fmt: verify\:format
.PHONY: verify\:fmt

format: verify\:format
.PHONY: format

fmt: verify\:format
.PHONY: fmt

verify\:lint: # Show suggestions relates to hygiene [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: verify\:lint
.PHONY: lint

verify\:all: err fmt lint # Run all [alias: verify]
.PHONY: verify\:all

verify: verify\:all
.PHONY: verify
# }}}

# build {{{
build\:debug: # Run exercises [alias: build]
	@cargo build
.PHONY: build\:debug

build: build\:debug
.PHONY: build

build\:debug\:%: # Build only specific binary like `ex1` [alias: build:%s]
	@cargo build --bin $(subst build:debug:,,$(@))
.PHONY: build\:debug\:%

build\:%:
	@make -s build:debug:ex1
.PHONY: build\:%
# }}}

# misc {{{
clean:
	@cargo clean
.PHONY: clean

help: # Display this message
	@grep --extended-regexp '^[0-9a-z\:\\\%]+: ' $(MAKEFILE_LIST) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-20s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help
# }}}

.DEFAULT_GOAL = verify\:all
default: verify\:all
