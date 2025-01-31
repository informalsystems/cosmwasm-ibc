# CHANGELOG

## v0.57.0

*January 31, 2025*

This release upgrades the `ibc-rs` dependencies to `v0.57.0`.

## v0.56.0

*November 15, 2024*

This release upgrades the `ibc-rs` and `tendermint-rs` dependencies to `v0.56.0`
and `v0.40.0`, respectively.

### BREAKING CHANGES
- Update `ibc` to `0.56.0` and `tendermint` to `0.40.0`.
  ([\#5](https://github.com/informalsystems/cosmwasm-ibc/pull/5))

## v0.55.0

*October 17, 2024*

This release upgrades the `ibc` dependencies to v0.55.1 and introduces
`CustomQuery` into the `Context` struct. Notably, the codebase has now been
migrated to its own repository,
[`cosmwasm-ibc`](https://github.com/informalsystems/cosmwasm-ibc).

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
