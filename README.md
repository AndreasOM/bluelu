# bluelu
Various build system related utils.


## Logging

Logging out is using log + env_logger,
so you can enable it by setting `RUST_LOG` to `info`, `trace`, `debug`, or `warn`.

e.g.
`RUST_LOG=debug bluelue-tea`


## Tools

### Tea - bluelue-tea

An improved version of classic unix `tee`.
Initially build to work around issues with windows/powershell `Tee-Object` which simply was unusable to replace `tee`.
