# usage
usage:
	@just -l

# prepare
prepare:
	@which sccache 2>&1 > /dev/null || cargo install sccache
	@cargo watch --help > /dev/null 2>&1 || cargo install cargo-watch
	@cargo audit --help > /dev/null 2>&1 || cargo install cargo-audit

# run
run: prepare
	cargo run

sccache_path := `which sccache`
# build
build: prepare
	RUSTC_WRAPPER={{sccache_path}} cargo build

# format
fmt: prepare
	@cargo fmt

# lint
lint: fmt
	@cargo clippy

# audit
audit: lint
	@cargo audit

# test
test: prepare
	cargo test

# watch
watch +COMMAND='test': prepare
	cargo watch --clear --exec "{{COMMAND}}"

alias boot := boot-local-cluster
# boot local cluster
boot-local-cluster:
	@if ! kind get clusters | grep biars 2>&1 > /dev/null; then \
		kind create cluster --name biars --config misc/kluster.yaml; \
	fi

# shutdown local cluster
shutdown-local-cluster:
	kind delete cluster --name biars

alias setup := applying-sample-application
# applying sample application
applying-sample-application: boot-local-cluster
	@git clone https://github.com/kubernetes/examples.git
	@kubectl apply -f examples/guestbook/all-in-one/guestbook-all-in-one.yaml
	@rm -rf examples

# clean
clean:
	@rm -rf examples
	@cargo clean

# clean all
clean-all: clean shutdown-local-cluster

# vim: set noexpandtab :
