default: build

all: test

test: build
	cargo test

build:
	soroban contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

deploy_alice_testnet: build
	soroban contract deploy \
  --wasm ./target/wasm32-unknown-unknown/release/github_oracle.wasm \
  --source alice \
  --network testnet
