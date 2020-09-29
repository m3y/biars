run:
	cargo run

build:
	cargo build

fmt:
	@cargo fmt

lint: fmt
	@cargo clippy

audit: lint
	@cargo audit

test:
	cargo test

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"

alias b := boot-local-cluster
boot-local-cluster:
	@if ! kind get clusters | grep cbias 2>&1 > /dev/null; then \
		kind create cluster --name cbias --config misc/kluster.yaml; \
	fi

shutdown-local-cluster:
	kind delete cluster --name cbias

alias setup := apply-sample-application
apply-sample-application: boot-local-cluster
	@git clone https://github.com/kubernetes/examples.git
	@kubectl apply -f examples/guestbook/all-in-one/guestbook-all-in-one.yaml
	@rm -rf examples

clean:
	@rm -rf examples
	@cargo clean

clean-all: clean shutdown-local-cluster

usage:
	@just -l

# vim: set noexpandtab :
