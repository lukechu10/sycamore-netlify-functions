.PHONY: build

help: ## Show this help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {sub("\\\\n",sprintf("\n%22c"," "), $$2);printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build static binary and put it in the functions directory.
	@cargo build --release --target x86_64-unknown-linux-musl --features lambda
	@mkdir -p functions
	@cp target/x86_64-unknown-linux-musl/release/sycamore-netlify-functions functions
