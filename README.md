# bluelu
Various build system related utils.

## Installation

```bash
cargo build --release
```

## Logging

Logging uses log + env_logger,
so you can enable it by setting `RUST_LOG` to `info`, `trace`, `debug`, or `warn`.

e.g.
```bash
RUST_LOG=debug bluelu-tea -b ls
```

## Tools

### Tea - bluelu-tea

An improved version of classic unix `tee`.
Initially built to work around issues with windows/powershell `Tee-Object` which simply was unusable to replace `tee`.

**Usage:**
```bash
bluelu-tea --binary <BINARY> [OPTIONS] [PARAMETERS]...

# Examples:
bluelu-tea -b echo -- hello world
bluelu-tea -o output.txt -b ls -- -la
```
