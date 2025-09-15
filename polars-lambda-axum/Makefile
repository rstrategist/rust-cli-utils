rust-version:
	@echo "Rust toolchain versions:"
	rustc --version
	cargo --version
	rustfmt --version
	rustup --version
	clippy-driver --version

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

# Local Axum server (port 3000)
run:
	cargo run

curl-json:
	curl -X GET http://127.0.0.1:3000/iris/filter/5

# Lambda runtime emulator (port 9000)
watch-lambda:
	LAMBDA_RUNTIME=1 cargo lambda watch --invoke-port 9000

invoke:
	cargo lambda invoke --data '{ "filter": 5.0 }'

# Build/deploy Lambda
build:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy --region us-east-1

release:
	cargo build --release

all: format lint test run
