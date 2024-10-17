# CHANGELOG

## v0.55.0

*October 17, 2024*

This release upgrades the `ibc` dependencies to v0.55.1 and introduces
`CustomQuery` into the `Context` struct. Notably, the codebase has now been
migrated to its own repository,
[`cosmwasm-ibc`](https://github.com/informalsystems/cosmwasm-ibc.git).

### BREAKING CHANGES

- Upgrade `ibc` dependencies to v0.55.1.
  ([\#3](https://github.com/informalsystems/cosmwasm-ibc/pull/3))

### FEATURES

- Allow `CustomQuery` in `Context`.
  ([\#1](https://github.com/informalsystems/cosmwasm-ibc/issues/1))

## Previous Releases

For lists of changes in previous releases, see the ibc-rs
[CHANGELOG](https://github.com/cosmos/ibc-rs/blob/main/CHANGELOG.md) before
v0.55.0.
