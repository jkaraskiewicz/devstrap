# Test Fixtures

This directory contains test-related files used for testing devstrap in isolated environments.

## Files

- **test-config.toml** - Minimal configuration file for testing
- **Dockerfile.test** - Docker image for testing devstrap in Ubuntu environment
- **Dockerfile.test-root** - Docker image for testing with root permissions

## Usage

### Running Tests with Docker

```bash
# Build test container
docker build -f tests/fixtures/Dockerfile.test-root -t devstrap-test .

# Run tests in container
docker run --rm devstrap-test /bin/bash -c \
  "apt-get update > /dev/null 2>&1 && \
   /root/devstrap/target/release/devstrap \
   --config tests/fixtures/test-config.toml --yes"
```

### Running with Custom Config

```bash
# Test with fixture config
cargo run -- --config tests/fixtures/test-config.toml --dry-run
```
