.PHONY: build run test clean fmt clippy doc update check release run-release docker-build docker-up docker-down docker-dev-up docker-build-optimized docker-build-chef docker-lint docker-scan

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy

# Generate documentation
doc:
	cargo doc

# Update dependencies
update:
	cargo update

# Check code without building
check:
	cargo check

# Build for release
release:
	cargo build --release

# Run in release mode
run-release:
	cargo run --release

# Build Docker images (standard)
docker-build:
	docker build -t listener:latest .

# Build Docker com BuildKit habilitado (mais rápido)
docker-build-optimized:
	DOCKER_BUILDKIT=1 docker build -t listener:latest .

# Build Docker usando cargo-chef (melhor cache)
docker-build-chef:
	DOCKER_BUILDKIT=1 docker build -f Dockerfile.chef -t listener:latest .

# Build Docker com versão específica
docker-build-version:
	DOCKER_BUILDKIT=1 docker build --build-arg APP_VERSION=$(shell git describe --tags --always --dirty) -t listener:$(shell git describe --tags --always --dirty) -t listener:latest .

# Lint Dockerfile
docker-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

# Scan vulnerabilidades na imagem
docker-scan:
	docker scan listener:latest

# Start services with docker-compose
docker-up:
	docker-compose up

# Stop services with docker-compose
docker-down:
	docker-compose down

# Start development services with docker-compose.dev.yml
docker-dev-up:
	docker-compose -f docker-compose.dev.yml up

# Build e start em um comando
docker-rebuild-up:
	DOCKER_BUILDKIT=1 docker-compose build && docker-compose up

# Logs dos containers
docker-logs:
	docker-compose logs -f

# Remove volumes e rebuild completo
docker-clean-rebuild:
	docker-compose down -v && DOCKER_BUILDKIT=1 docker-compose build --no-cache
