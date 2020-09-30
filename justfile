# run
run:
	cargo run

# build
build:
	cargo build

# format
fmt:
	@cargo fmt

# lint
lint: fmt
	@cargo clippy

# audit
audit: lint
	@cargo audit

# test
test:
	cargo test

# watch
watch +COMMAND='test':
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

# usage
usage:
	@just -l

# vim: set noexpandtab :
